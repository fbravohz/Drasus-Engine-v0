mod desktop_core;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    let app = tauri::Builder::default()
        .setup(|app| {
            app.manage(desktop_core::DesktopBridge::start(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            desktop_core::get_core_status,
            desktop_core::retry_core_connection,
            desktop_core::close_desktop_window
        ])
        .on_window_event(desktop_core::on_window_event)
        .build(tauri::generate_context!())?;

    app.run(|app, event| {
        if matches!(event, tauri::RunEvent::Exit) {
            desktop_core::request_shutdown(app);
        }
    });
    Ok(())
}
