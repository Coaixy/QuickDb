#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quickdb::cmds;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmds::read_xlsx,cmds::get_app_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
