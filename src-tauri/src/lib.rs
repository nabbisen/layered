use tauri::Manager;

mod core;
use core::editor::{compose, parse, ready};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ready, parse, compose])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
