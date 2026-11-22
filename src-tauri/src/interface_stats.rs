use windows_sys::{core::GUID, Win32::NetworkManagement::IpHelper::{ConvertInterfaceGuidToLuid, GetBestInterface, GetIfEntry2, MIB_IF_ROW2}};

#[derive(serde::Serialize)]
pub struct InterfaceCounters { pub received: u64, pub sent: u64 }

fn parse_guid(interface: &str) -> Result<GUID, String> {
    let start = interface.find('{').ok_or("La interfaz no tiene un identificador compatible")?;
    let suffix = &interface[start..];
    if !suffix.is_ascii() || !suffix.ends_with('}') || suffix.len() != 38 { return Err("Identificador de interfaz invalido".into()); }
    let parts: Vec<_> = suffix[1..37].split('-').collect();
    if parts.len() != 5 || [8,4,4,4,12].iter().zip(&parts).any(|(n,p)| p.len() != *n || !p.bytes().all(|b| b.is_ascii_hexdigit())) { return Err("Identificador de interfaz invalido".into()); }
    let parse = |s: &str| u64::from_str_radix(s,16).map_err(|_| "Identificador de interfaz invalido".to_string());
    let bytes = format!("{}{}", parts[3], parts[4]);
    let mut data4 = [0u8; 8];
    for (i, item) in data4.iter_mut().enumerate() { *item = parse(&bytes[i*2..i*2+2])? as u8; }
    Ok(GUID { data1: parse(parts[0])? as u32, data2: parse(parts[1])? as u16, data3: parse(parts[2])? as u16, data4 })
}
pub(crate) fn interface_index(interface: &str) -> Result<u32, String> {
    let guid = parse_guid(interface)?;
    let mut row = MIB_IF_ROW2::default();
    let code = unsafe { ConvertInterfaceGuidToLuid(&guid, &mut row.InterfaceLuid) };
    if code != 0 { return Err(format!("No se puede identificar la interfaz (Windows {code})")); }
    let code = unsafe { GetIfEntry2(&mut row) };
    if code != 0 { return Err(format!("No se puede consultar la interfaz (Windows {code})")); }
    if row.OperStatus != windows_sys::Win32::NetworkManagement::Ndis::IfOperStatusUp { return Err("La interfaz de salida no esta activa".into()); }
    Ok(row.InterfaceIndex)
}
pub(crate) fn active_route_index() -> Result<u32, String> {
    let mut index = 0;
    // Consulta local de ruta: no se envian paquetes al destino de referencia.
    let code = unsafe { GetBestInterface(u32::from_ne_bytes([1,1,1,1]), &mut index) };
    if code != 0 || index == 0 { return Err("Windows no encontro una ruta IPv4 de salida activa".into()); }
    Ok(index)
}
#[tauri::command]
pub fn active_interface() -> Result<crate::capture::providers::npcap::Interface, String> {
    let index = active_route_index()?;
    crate::capture::providers::npcap::Api::load()?.interfaces()?.into_iter()
        .find(|device| interface_index(&device.id).ok() == Some(index))
        .ok_or_else(|| "La interfaz que Windows usa para la ruta IPv4 no esta disponible en Npcap".into())
}
#[tauri::command]
pub fn interface_counters(interface: String) -> Result<InterfaceCounters, String> {
    let guid = parse_guid(&interface)?;
    let mut row = MIB_IF_ROW2::default();
    let code = unsafe { ConvertInterfaceGuidToLuid(&guid, &mut row.InterfaceLuid) };
    if code != 0 { return Err(format!("No se puede identificar la interfaz (Windows {code})")); }
    let code = unsafe { GetIfEntry2(&mut row) };
    if code != 0 { return Err(format!("No se pueden consultar sus contadores (Windows {code})")); }
    Ok(InterfaceCounters { received: row.InOctets, sent: row.OutOctets })
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn guid_validation() {
        let guid = parse_guid(r"\Device\NPF_{00112233-4455-6677-8899-AABBCCDDEEFF}").unwrap();
        assert_eq!(guid.data1, 0x00112233); assert_eq!(guid.data4, [0x88,0x99,0xaa,0xbb,0xcc,0xdd,0xee,0xff]);
        assert!(parse_guid("invalid").is_err()); assert!(parse_guid("{00112233-4455-6677-8899-AABBCCDDEEFZ}").is_err());
    }
}
