use std::ops::Add;
use calamine::{DataType, open_workbook, Reader, Xlsx};

pub struct DataObject {
    pub data: Vec<Vec<String>>,
}

impl DataObject {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }
    pub fn read(&mut self, path: &str) {
        let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
        let range = workbook.worksheet_range("Sheet1").unwrap();
        let mut row_vec: Vec<String> = Vec::new();
        for row in range.rows() {
            for cell in row.iter() {
                row_vec.push(format!("{:?}", cell.to_string()));
            }
            self.data.push(row_vec.clone());
            row_vec.clear();
        }
    }
}
