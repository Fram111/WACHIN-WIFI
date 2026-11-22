use std::{collections::VecDeque, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc}, thread, time::{Duration, Instant}};
use tauri::{AppHandle, Emitter, State};
use crate::{capture::providers::npcap::{Api, Interface}, parser};

const MEMORY_LIMIT: usize = 2000;
#[derive(Clone, serde::Serialize)]
pub struct Packet { id: u64, timestamp_ms: i64, length: u32, captured_length: usize, source: Option<String>, destination: Option<String>, protocol: &'static str, source_port: Option<u16>, destination_port: Option<u16>, truncated: bool, fragmented: bool }
struct StoredPacket { packet: Packet, bytes: Vec<u8> }
#[derive(Clone, serde::Serialize)]
pub struct Snapshot { active: bool, captured: u64, bytes: u64, evicted: u64, error: Option<String> }
struct Inner { snapshot: Snapshot, packets: VecDeque<StoredPacket>, stop: Option<Arc<AtomicBool>> }
pub struct Engine(Mutex<Inner>);
impl Default for Engine { fn default() -> Self { Self(Mutex::new(Inner { snapshot: Snapshot { active: false, captured: 0, bytes: 0, evicted: 0, error: None }, packets: VecDeque::new(), stop: None })) } }
#[derive(Clone, serde::Serialize)] pub(crate) struct Batch { packets: Vec<Packet>, status: Snapshot }
#[derive(serde::Serialize)] pub struct Detail { packet: Packet, hex: String, ascii: String }

#[tauri::command] pub fn list_interfaces() -> Result<Vec<Interface>, String> { Api::load()?.interfaces() }
#[tauri::command] pub fn capture_status(engine: State<'_, Arc<Engine>>) -> Result<Snapshot, String> { Ok(engine.0.lock().map_err(|_| "Estado inaccesible")?.snapshot.clone()) }
#[tauri::command] pub fn packet_detail(id: u64, engine: State<'_, Arc<Engine>>) -> Result<Detail, String> {
    let state = engine.0.lock().map_err(|_| "Estado inaccesible")?;
    let item = state.packets.iter().find(|p| p.packet.id == id).ok_or("Paquete fuera del límite de memoria")?;
    let hex = item.bytes.chunks(16).enumerate().map(|(n, row)| format!("{:04x}  {}", n * 16, row.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join("\n");
    let ascii = item.bytes.chunks(16).map(|row| row.iter().map(|b| if (0x20..=0x7e).contains(b) { *b as char } else { '.' }).collect::<String>()).collect::<Vec<_>>().join("\n");
    Ok(Detail { packet: item.packet.clone(), hex, ascii })
}
#[tauri::command] pub fn stop_capture(engine: State<'_, Arc<Engine>>) -> Result<(), String> { let state = engine.0.lock().map_err(|_| "Estado inaccesible")?; if let Some(stop) = &state.stop { stop.store(true, Ordering::Release); } Ok(()) }
#[tauri::command] pub fn start_capture(interface: String, app: AppHandle, engine: State<'_, Arc<Engine>>) -> Result<(), String> {
    let expected_index = crate::interface_stats::interface_index(&interface)?;
    if crate::interface_stats::active_route_index()? != expected_index { return Err("La interfaz activa cambio; actualiza la interfaz antes de iniciar".into()); }
    let shared = Arc::clone(&engine); let stop = Arc::new(AtomicBool::new(false));
    { let mut state = shared.0.lock().map_err(|_| "Estado inaccesible")?; if state.snapshot.active { return Err("Ya hay una captura activa".into()); } state.snapshot = Snapshot { active: true, captured: 0, bytes: 0, evicted: 0, error: None }; state.packets.clear(); state.stop = Some(Arc::clone(&stop)); }
    let (tx, rx) = mpsc::sync_channel(1);
    let timeout_stop = Arc::clone(&stop);
    thread::spawn(move || {
        let mut capture = match Api::load().and_then(|api| api.open(&interface)) { Ok(capture) => { let _ = tx.send(Ok(())); capture }, Err(err) => { if let Ok(mut state) = shared.0.lock() { state.snapshot.active = false; state.snapshot.error = Some(err.clone()); state.stop = None; } let _ = tx.send(Err(err)); return; } };
        let mut batch = Vec::with_capacity(128); let mut last = Instant::now(); let mut route_check = Instant::now(); let mut error = None;
        while !stop.load(Ordering::Acquire) {
            if route_check.elapsed() >= Duration::from_secs(3) {
                route_check = Instant::now();
                if crate::interface_stats::active_route_index().ok() != Some(expected_index) || crate::interface_stats::interface_index(&interface).ok() != Some(expected_index) {
                    error = Some("La ruta activa cambio o dejo de estar disponible. La captura se detuvo; actualiza la interfaz e inicia otra sesion.".into()); break;
                }
            }
            match capture.next() {
                Ok(Some(raw)) => { let decoded = parser::parse_ethernet(&raw.data); let mut state = match shared.0.lock() { Ok(s) => s, Err(_) => break }; state.snapshot.captured += 1; state.snapshot.bytes += u64::from(raw.length); let packet = Packet { id: state.snapshot.captured, timestamp_ms: raw.timestamp_ms, length: raw.length, captured_length: raw.data.len(), source: decoded.source, destination: decoded.destination, protocol: decoded.protocol, source_port: decoded.source_port, destination_port: decoded.destination_port, truncated: decoded.truncated || raw.data.len() < raw.length as usize, fragmented: decoded.fragmented }; if state.packets.len() == MEMORY_LIMIT { state.packets.pop_front(); state.snapshot.evicted += 1; } state.packets.push_back(StoredPacket { packet: packet.clone(), bytes: raw.data }); batch.push(packet); },
                Ok(None) => thread::sleep(Duration::from_millis(10)), Err(e) => { error = Some(e); break; }
            }
            if batch.len() >= 128 || last.elapsed() >= Duration::from_millis(200) { if let Ok(state) = shared.0.lock() { let _ = app.emit("capture-batch", Batch { packets: std::mem::take(&mut batch), status: state.snapshot.clone() }); } last = Instant::now(); }
        }
        if let Ok(mut state) = shared.0.lock() { state.snapshot.active = false; state.snapshot.error = error; state.stop = None; let _ = app.emit("capture-batch", Batch { packets: batch, status: state.snapshot.clone() }); }
    });
    match rx.recv_timeout(Duration::from_secs(10)) { Ok(result) => result, Err(_) => { timeout_stop.store(true, Ordering::Release); Err("El proveedor tardó demasiado en iniciar".into()) } }
}


#[derive(serde::Serialize)]
pub struct HistoryEntry { id: i64, created_ms: i64, captured: i64, stored: i64, evicted: i64, bytes: i64 }
fn history_db(app: &AppHandle) -> Result<(rusqlite::Connection, std::path::PathBuf), String> {
    use tauri::Manager;
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?.join("history");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(dir.join("history.sqlite3")).map_err(|e| e.to_string())?;
    conn.execute_batch("CREATE TABLE IF NOT EXISTS sessions (id INTEGER PRIMARY KEY AUTOINCREMENT, created_ms INTEGER NOT NULL, captured INTEGER NOT NULL, stored INTEGER NOT NULL, evicted INTEGER NOT NULL, bytes INTEGER NOT NULL)").map_err(|e| e.to_string())?;
    Ok((conn, dir))
}
fn history_path(dir: &std::path::Path, id: i64) -> Result<std::path::PathBuf, String> {
    if id < 1 { return Err("Sesion invalida".into()); }
    Ok(dir.join(format!("session-{id}.pcap")))
}
fn has_history(conn: &rusqlite::Connection, id: i64) -> Result<bool, String> {
    use rusqlite::OptionalExtension;
    Ok(conn.query_row("SELECT id FROM sessions WHERE id=?1", [id], |r| r.get::<_, i64>(0)).optional().map_err(|e| e.to_string())?.is_some())
}
#[tauri::command] pub fn list_history(app: AppHandle) -> Result<Vec<HistoryEntry>, String> {
    let (conn, _) = history_db(&app)?;
    let mut stmt = conn.prepare("SELECT id,created_ms,captured,stored,evicted,bytes FROM sessions ORDER BY id DESC").map_err(|e| e.to_string())?;
    let entries = stmt.query_map([], |r| Ok(HistoryEntry { id:r.get(0)?, created_ms:r.get(1)?, captured:r.get(2)?, stored:r.get(3)?, evicted:r.get(4)?, bytes:r.get(5)? })).map_err(|e| e.to_string())?;
    entries.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}
#[tauri::command] pub fn save_session(app: AppHandle, engine: State<'_, Arc<Engine>>) -> Result<i64, String> {
    use crate::storage::{Frame, write_pcap};
    let state = engine.0.lock().map_err(|_| "Estado inaccesible")?;
    if state.snapshot.active { return Err("Deten la captura antes de guardarla".into()); }
    if state.packets.is_empty() { return Err("No hay paquetes para guardar".into()); }
    let frames: Vec<_> = state.packets.iter().map(|p| Frame { timestamp_ms:p.packet.timestamp_ms, original_len:p.packet.length, bytes:p.bytes.clone() }).collect();
    let snapshot = state.snapshot.clone(); drop(state);
    let created_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_millis() as i64;
    let (mut conn, dir) = history_db(&app)?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    tx.execute("INSERT INTO sessions(created_ms,captured,stored,evicted,bytes) VALUES(?1,?2,?3,?4,?5)", rusqlite::params![created_ms, snapshot.captured as i64, frames.len() as i64, snapshot.evicted as i64, snapshot.bytes as i64]).map_err(|e| e.to_string())?;
    let id = tx.last_insert_rowid(); let path = history_path(&dir,id)?;
    let file = std::fs::OpenOptions::new().write(true).create_new(true).open(&path).map_err(|e| e.to_string())?;
    if let Err(e) = write_pcap(std::io::BufWriter::new(file), &frames) { let _ = std::fs::remove_file(&path); return Err(e); }
    if let Err(e) = tx.commit() { let _ = std::fs::remove_file(&path); return Err(e.to_string()); }
    Ok(id)
}
#[tauri::command] pub fn delete_session(id: i64, app: AppHandle) -> Result<(), String> {
    let (conn, dir) = history_db(&app)?; if !has_history(&conn,id)? { return Err("La sesion no existe".into()); }
    let path = history_path(&dir,id)?;
    match std::fs::remove_file(path) { Ok(()) => (), Err(e) if e.kind() == std::io::ErrorKind::NotFound => (), Err(e) => return Err(e.to_string()) }
    conn.execute("DELETE FROM sessions WHERE id=?1", [id]).map_err(|e| e.to_string())?; Ok(())
}
#[tauri::command] pub fn open_session(id: i64, app: AppHandle, engine: State<'_, Arc<Engine>>) -> Result<Batch, String> {
    let (conn, dir) = history_db(&app)?;
    let saved: (u64,u64,u64) = conn.query_row("SELECT captured,evicted,bytes FROM sessions WHERE id=?1", [id], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?))).map_err(|_| "La sesion no existe")?;
    let frames = crate::storage::read_pcap(std::io::BufReader::new(std::fs::File::open(history_path(&dir,id)?).map_err(|e| e.to_string())?))?;
    let mut state = engine.0.lock().map_err(|_| "Estado inaccesible")?;
    if state.snapshot.active { return Err("Deten la captura antes de abrir una sesion".into()); }
    state.packets.clear();
    for (index, raw) in frames.into_iter().enumerate() {
        let decoded = parser::parse_ethernet(&raw.bytes);
        let packet = Packet { id: index as u64 + 1, timestamp_ms: raw.timestamp_ms, length:raw.original_len, captured_length:raw.bytes.len(), source:decoded.source, destination:decoded.destination, protocol:decoded.protocol, source_port:decoded.source_port, destination_port:decoded.destination_port, truncated:decoded.truncated || raw.bytes.len() < raw.original_len as usize, fragmented:decoded.fragmented };
        state.packets.push_back(StoredPacket { packet, bytes:raw.bytes });
    }
    state.snapshot = Snapshot { active:false, captured:saved.0, evicted:saved.1, bytes:saved.2, error:None };
    Ok(Batch { packets:state.packets.iter().map(|p| p.packet.clone()).collect(), status:state.snapshot.clone() })
}
#[tauri::command] pub async fn export_session(id: i64, app: AppHandle) -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(move || {
        use tauri_plugin_dialog::DialogExt;
        let (conn, dir) = history_db(&app)?; if !has_history(&conn,id)? { return Err("La sesion no existe".to_string()); }
        let source = history_path(&dir,id)?;
        let Some(destination) = app.dialog().file().add_filter("Captura PCAP", &["pcap"]).set_file_name(format!("wachin-sesion-{id}.pcap")).blocking_save_file() else { return Ok(false); };
        let destination = destination.into_path().map_err(|e| e.to_string())?;
        let history_root = dir.canonicalize().map_err(|e| e.to_string())?;
        let parent = destination.parent().ok_or("Destino invalido")?.canonicalize().map_err(|e| e.to_string())?;
        if parent.starts_with(&history_root) || destination.canonicalize().is_ok_and(|p| p.starts_with(&history_root)) { return Err("Selecciona un destino fuera del historial de la aplicacion".into()); }
        std::fs::copy(source, destination).map_err(|e| e.to_string())?; Ok(true)
    }).await.map_err(|e| e.to_string())?
}
