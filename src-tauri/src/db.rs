use crate::utils;
use calamine::Rows;
use rusqlite::{Connection, Row};
use std::{string::String, vec};
use uuid::Uuid;

pub struct Db {
    conn: Connection,
}

#[derive(Debug)]
struct BalanceItem {
    uuid: String,
    phone: String,
    balance: String,
    create_time: String,
}

impl Db {
    pub fn new() -> Self {
        Self {
            conn: Self::open_db(),
        }
    }
    pub fn open_db() -> Connection {
        let path = format!("{}/{}", utils::app_path(), "quickdb.sqlite");
        let conn = Connection::open(path).unwrap();
        conn
    }
    pub fn get_table_data(&mut self, table: String, phone: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if phone == "cache" && table == "cache" {
            let sql = "SELECT * FROM cache";
            let mut stmt = self.conn.prepare(sql).unwrap();
            let mut balance_item_row = stmt.query([]).unwrap();
            while let Some(row) = balance_item_row.next().unwrap() {
                let time: String = row.get(1).unwrap();
                if !result.contains(&time) {
                    result.push(time);
                }
            }
        } else {
            let params = [&phone];
            let mut stmt = self
                .conn
                .prepare(format!("select * from {} where phone = ?1", table).as_str())
                .unwrap();
            let mut balance_item_row = stmt.query(&params).unwrap();
            while let Some(row) = balance_item_row.next().unwrap() {
                result.push(row.get(0).unwrap());
                result.push(phone);
                result.push(row.get(2).unwrap());
                result.push(row.get(3).unwrap());
                break;
            }
        }
        result
    }
    pub fn out_data(&mut self, table: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut stmt = self
            .conn
            .prepare(format!("select * from {}", table).as_str())
            .unwrap();
        let mut item_rows = stmt.query([]).unwrap();
        if table == "cache" {
            while let Some(row) = item_rows.next().unwrap() {
                let mut item: Vec<String> = Vec::new();
                item.push(row.get(1).unwrap());
                result.push(item);
            }
        } else {
            while let Some(row) = item_rows.next().unwrap() {
                let mut item: Vec<String> = Vec::new();
                item.push(row.get(0).unwrap());
                item.push(row.get(1).unwrap());
                item.push(row.get(2).unwrap());
                item.push(row.get(3).unwrap());
                result.push(item);
            }
        }
        result
    }
    pub fn save_data(&mut self, table: String, phone: String, data: Vec<String>) -> usize {
        let mut table_data: Vec<String> = Vec::new();
        if table == "cache" {
            table_data = self.get_table_data(table.clone(), String::from("cache"));
        } else {
            table_data = self.get_table_data(table.clone(), phone.clone());
        }
        let uuid = Uuid::new_v4().to_string();
        println!("{}-{}-{:?}", table, phone, data);
        println!("table_data : {:?}", table_data);
        match table.as_str() {
            "balance" => {
                if table_data.len() <= 0 {
                    let mut stmt = self.conn.prepare("INSERT INTO balance (uuid,phone,balance,create_time,daili,fangan,celue,choujin) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)").unwrap();
                    return stmt
                        .execute(&[
                            &uuid, &phone, &data[0], &data[1], &data[2], &data[3], &data[4],
                            &data[5],
                        ])
                        .unwrap()
                        .try_into()
                        .unwrap();
                } else if table_data.len() > 0 {
                    let new_balance = data[0].replace("\"", "").parse::<f32>().unwrap()
                        + table_data[2].replace("\"", "").parse::<f32>().unwrap();
                    let mut stmt = self
                        .conn
                        .prepare("UPDATE balance SET balance = ?1 WHERE phone = ?2")
                        .unwrap();
                    return stmt.execute([new_balance.to_string(), phone]).unwrap();
                }
            }
            "timer" => {
                // if table_data.len() <= 0 {
                let mut stmt = self
                    .conn
                    .prepare("INSERT INTO timer (uuid,phone,time,money) VALUES (?1,?2,?3,?4)")
                    .unwrap();
                return stmt.execute(&[&uuid, &phone, &data[0], &data[1]]).unwrap();
                // }
            }
            "cache" => {
                if !table_data.contains(&data[0]) {
                    let mut stmt = self
                        .conn
                        .prepare("INSERT INTO cache (uuid,time) VALUES (?1,?2)")
                        .unwrap();
                    return stmt.execute(&[&uuid, &data[0]]).unwrap();
                }
            }
            _ => {
                return 0;
            }
        }
        0
    }
}
