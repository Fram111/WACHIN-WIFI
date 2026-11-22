//! Carga explícita de Npcap; no se redistribuye ni instala el controlador.
use libloading::Library;
use std::{ffi::{CStr, CString, OsString, c_char, c_int, c_uchar, c_void}, os::windows::ffi::OsStringExt, path::PathBuf, ptr};

#[link(name = "kernel32")] unsafe extern "system" { fn GetSystemDirectoryW(buffer: *mut u16, size: u32) -> u32; }

#[repr(C)] struct Device { next: *mut Device, name: *mut c_char, description: *mut c_char, addresses: *mut c_void, flags: u32 }
#[repr(C)] struct Timeval { sec: i32, usec: i32 }
#[repr(C)] struct Header { ts: Timeval, caplen: u32, len: u32 }
type Find = unsafe extern "C" fn(*mut *mut Device, *mut c_char) -> c_int;
type Free = unsafe extern "C" fn(*mut Device);
type Open = unsafe extern "C" fn(*const c_char, c_int, c_int, c_int, *mut c_char) -> *mut c_void;
type Next = unsafe extern "C" fn(*mut c_void, *mut *const Header, *mut *const c_uchar) -> c_int;
type Close = unsafe extern "C" fn(*mut c_void);
type Datalink = unsafe extern "C" fn(*mut c_void) -> c_int;
type SetNonblock = unsafe extern "C" fn(*mut c_void, c_int, *mut c_char) -> c_int;
type Geterr = unsafe extern "C" fn(*mut c_void) -> *const c_char;

pub struct Api { _library: Library, find: Find, free: Free, open: Open, next: Next, close: Close, datalink: Datalink, nonblock: SetNonblock, geterr: Geterr }
#[derive(serde::Serialize)] pub struct Interface { pub id: String, pub name: String }
pub struct RawPacket { pub timestamp_ms: i64, pub length: u32, pub data: Vec<u8> }
pub struct Capture { api: Api, handle: *mut c_void }

impl Api {
    pub fn load() -> Result<Self, String> {
        let mut buffer = [0u16; 32768];
        let len = unsafe { GetSystemDirectoryW(buffer.as_mut_ptr(), buffer.len() as u32) } as usize;
        if len == 0 || len >= buffer.len() { return Err("No se pudo obtener el directorio del sistema".into()); }
        let path = PathBuf::from(OsString::from_wide(&buffer[..len])).join("Npcap").join("wpcap.dll");
        // LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR also resolves Packet.dll from Npcap's own directory.
        let lib = unsafe { libloading::os::windows::Library::load_with_flags(&path, 0x00000100 | 0x00000800) }
            .map_err(|_| "Npcap no está disponible. Instala Npcap por separado desde npcap.com y reinicia la aplicación.".to_string())?;
        let library: Library = lib.into();
        unsafe {
            macro_rules! symbol { ($name:literal, $ty:ty) => { *library.get::<$ty>(concat!($name, "\0").as_bytes()).map_err(|e| format!("Npcap incompatible: {e}"))? }; }
            Ok(Self { find: symbol!("pcap_findalldevs", Find), free: symbol!("pcap_freealldevs", Free), open: symbol!("pcap_open_live", Open), next: symbol!("pcap_next_ex", Next), close: symbol!("pcap_close", Close), datalink: symbol!("pcap_datalink", Datalink), nonblock: symbol!("pcap_setnonblock", SetNonblock), geterr: symbol!("pcap_geterr", Geterr), _library: library })
        }
    }
    pub fn interfaces(&self) -> Result<Vec<Interface>, String> {
        let mut head = ptr::null_mut(); let mut error = [0 as c_char; 256];
        if unsafe { (self.find)(&mut head, error.as_mut_ptr()) } != 0 { return Err(unsafe { CStr::from_ptr(error.as_ptr()) }.to_string_lossy().into_owned()); }
        let mut out = Vec::new(); let mut cursor = head;
        while !cursor.is_null() { unsafe { let item = &*cursor; if !item.name.is_null() { let id = CStr::from_ptr(item.name).to_string_lossy().into_owned(); let name = if item.description.is_null() { id.clone() } else { CStr::from_ptr(item.description).to_string_lossy().into_owned() }; out.push(Interface { id, name }); } cursor = item.next; } }
        unsafe { (self.free)(head) }; Ok(out)
    }
    pub fn open(self, id: &str) -> Result<Capture, String> {
        if !self.interfaces()?.iter().any(|i| i.id == id) { return Err("La interfaz seleccionada ya no está disponible".into()); }
        let id = CString::new(id).map_err(|_| "Identificador de interfaz inválido")?; let mut error = [0 as c_char; 256];
        // Promiscuous mode disabled; capture remains limited to traffic delivered by the adapter.
        let handle = unsafe { (self.open)(id.as_ptr(), 65535, 0, 250, error.as_mut_ptr()) };
        if handle.is_null() { return Err(unsafe { CStr::from_ptr(error.as_ptr()) }.to_string_lossy().into_owned()); }
        if unsafe { (self.datalink)(handle) } != 1 { unsafe { (self.close)(handle) }; return Err("Esta interfaz no proporciona tramas Ethernet compatibles".into()); }
        if unsafe { (self.nonblock)(handle, 1, error.as_mut_ptr()) } != 0 { unsafe { (self.close)(handle) }; return Err(unsafe { CStr::from_ptr(error.as_ptr()) }.to_string_lossy().into_owned()); }
        Ok(Capture { api: self, handle })
    }
}
impl Capture {
    pub fn next(&mut self) -> Result<Option<RawPacket>, String> {
        let mut hdr = ptr::null(); let mut bytes = ptr::null(); let code = unsafe { (self.api.next)(self.handle, &mut hdr, &mut bytes) };
        match code { 0 => Ok(None), 1 if !hdr.is_null() && !bytes.is_null() => unsafe { let h = &*hdr; if h.caplen > 65535 || h.caplen > h.len { return Err("Npcap devolvió una longitud inválida".into()); } Ok(Some(RawPacket { timestamp_ms: i64::from(h.ts.sec)*1000 + i64::from(h.ts.usec)/1000, length: h.len, data: std::slice::from_raw_parts(bytes, h.caplen as usize).to_vec() })) }, _ => { let msg = unsafe { (self.api.geterr)(self.handle) }; Err(if msg.is_null() { "Error de lectura de Npcap".into() } else { unsafe { CStr::from_ptr(msg) }.to_string_lossy().into_owned() }) } }
    }
}
impl Drop for Capture { fn drop(&mut self) { unsafe { (self.api.close)(self.handle) } } }

