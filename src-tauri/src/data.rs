use std::collections::BTreeMap;
use std::collections::HashMap;

use calamine::{open_workbook, Reader, Xlsx};

use crate::db::Db;

pub struct DataObject {
    pub data: Vec<Vec<String>>,
    obj: Db,
}

impl DataObject {
    pub fn new() -> Self {
        Self {
            data: vec![],
            obj: Db::new(),
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
    pub fn save_balance(&mut self) {
        let balance_all = self.obj.out_data("balance".to_owned());
        //(uuid,phone,balance,create_time,daili,fangan,celue,choujin)
        let mut index_map: HashMap<String, i32> = HashMap::new();
        if self.data.len() > 0 {
            for (index, value) in self.data.iter().enumerate() {
                if index == 0 {
                    for (t, ti) in value.iter().enumerate() {
                        match ti.as_str() {
                            "\"号码\"" => {
                                index_map.insert("号码".to_owned(), t as i32);
                            }
                            "\"方案名称\"" => {
                                index_map.insert("方案".to_owned(), t as i32);
                            }
                            "\"策略名称\"" => {
                                index_map.insert("策略".to_owned(), t as i32);
                            }
                            "\"代理商名称\"" => {
                                index_map.insert("代理".to_owned(), t as i32);
                            }
                            "\"酬金类型\"" => {
                                index_map.insert("酬金".to_owned(), t as i32);
                            }
                            "\"账期\"" => {
                                index_map.insert("账期".to_owned(), t as i32);
                            }
                            "\"佣金（元）\"" => {
                                index_map.insert("佣金".to_owned(), t as i32);
                            }
                            _ => {}
                        }
                    }
                    println!("{:?}", index_map);
                } else {
                    let zhangqi = value[index_map.get("账期").unwrap().clone() as usize].clone().replace("\"", "");
                    let phone = value[index_map.get("号码").unwrap().clone() as usize].clone().replace("\"", "");
                    let fangan = value[index_map.get("方案").unwrap().clone() as usize].clone().replace("\"", "");
                    let celue = value[index_map.get("策略").unwrap().clone() as usize].clone().replace("\"", "");
                    let daili = value[index_map.get("代理").unwrap().clone() as usize].clone().replace("\"", "");
                    let choujin = value[index_map.get("酬金").unwrap().clone() as usize].clone().replace("\"", "");
                    let yongjin = value[index_map.get("佣金").unwrap().clone() as usize].clone().replace("\"", "");
                    
                    let data = [
                        yongjin.to_owned(),
                        zhangqi.to_owned(),
                        daili.to_owned(),
                        fangan.to_owned(),
                        celue.to_owned(),
                        choujin.to_owned(),
                    ];
                    self.obj
                        .save_data("balance".to_string(), phone.clone(), data.to_vec());

                    let cache_all = self.obj.out_data("cache".to_owned());
                    let mut cache_flag = false;
                    for cache_item in cache_all.iter() {
                        if cache_item[0] == zhangqi {
                            cache_flag = true;
                            break;
                        }
                    }
                    if !cache_flag {
                        let data = [zhangqi.to_owned()];
                        self.obj
                            .save_data("cache".to_string(), phone.clone(), data.to_vec());
                    }

                    let mut timer_flag = false;
                    let timer_all = self.obj.out_data("timer".to_owned());
                    for timer_item in timer_all.iter() {
                        // println!("{} = {} , {} = {}",timer_item[1],zhangqi,timer_item[3],phone);
                        if timer_item[1] == zhangqi && timer_item[3] == phone {
                            timer_flag = true;
                            break;
                        }
                    }
                    if !timer_flag {
                        let data = [zhangqi.to_owned(), yongjin.to_owned()];
                        self.obj
                            .save_data("timer".to_string(), phone, data.to_vec());
                    }
                }
            }
        }
    }
}
