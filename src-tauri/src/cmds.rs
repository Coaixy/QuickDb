use crate::data::DataObject;
use crate::{db, utils};

#[tauri::command]
pub fn read_xlsx(path: String) -> Vec<String> {
    // let mut o = DataObject::new();
    // o.read(&path);
    // return o.data;
    let mut o = db::Db::new();
    o.get_table_data("cache".to_string(), "cache".to_string())
}

#[tauri::command]
pub fn get_app_path() -> String {
    return utils::app_path();
}