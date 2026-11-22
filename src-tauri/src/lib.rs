pub mod capture;
pub mod parser;
pub mod storage;
#[cfg(windows)] mod engine;
#[cfg(windows)] mod interface_stats;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(windows)]
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(std::sync::Arc::new(engine::Engine::default()))
        .invoke_handler(tauri::generate_handler![engine::list_interfaces, engine::start_capture, engine::stop_capture, engine::capture_status, engine::packet_detail, interface_stats::interface_counters, interface_stats::active_interface, engine::list_history, engine::save_session, engine::delete_session, engine::open_session, engine::export_session]);
    #[cfg(not(windows))]
    let builder = tauri::Builder::default();
    builder.run(tauri::generate_context!()).expect("error al iniciar WACHIN-WIFI");
}
