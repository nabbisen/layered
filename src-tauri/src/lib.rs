use tauri::Manager;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod core;

use core::types;
use core::utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn save(contents: Vec<types::ContentType>) -> Result<(), String> {
    // println!("{:?}", contents);
    let contenst_str = format!("{}\n", utils::contents_str(&contents, 1));
    // println!("{}", contenst_str);

    let home_path =
    // Linux / Unix / Mac
    env::var("HOME")
        .ok()
        // Windows
        .or_else(|| env::var("USERPROFILE").ok());
    let mut filepath = PathBuf::from(home_path.unwrap());
    // Linux / Unix / Mac / Windows
    filepath.push("Desktop");
    filepath.push("layered-saved.md");
    filepath.to_string_lossy().into_owned();
    File::create(filepath)
        .and_then(|mut file| file.write_all(contenst_str.as_bytes()))
        .map_err(|err| format!("Failed to save file: {}", err))
        .unwrap();
    Ok(())
}

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
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
