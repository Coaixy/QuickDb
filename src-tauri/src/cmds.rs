use crate::data::DataObject;
use crate::{db, utils};

#[tauri::command]
pub fn read_xlsx(path: String) {
    let mut data = DataObject::new();
    data.read(&path);
    data.save_balance();
}

#[tauri::command]
pub fn get_app_path() -> String {
    return utils::app_path();
}