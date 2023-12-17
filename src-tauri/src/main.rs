#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calamine::DataType;
use quickdb::data::DataObject;

#[tauri::command]
fn hello(path: String) -> Vec<Vec<String>> {
    let mut o = DataObject::new();
    o.read(&path);
    return o.data;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
