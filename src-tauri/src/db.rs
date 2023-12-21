use crate::utils;
use calamine::Rows;
use rusqlite::{Connection, Row};
use std::string::String;
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
        let mut sql = "SELECT * FROM ?1 WHERE phone = ?2";
        if phone == "cache" && table == "cache"{
            sql = "SELECT * FROM cache";
            let mut stmt = self.conn.prepare(sql).unwrap();
            let mut balance_item_row = stmt.query([]).unwrap();
            while let Some(row) = balance_item_row.next().unwrap() {
                let uuid: String = row.get(0).unwrap();
                let time: String = row.get(1).unwrap();
                let temp = format!("{}-{}", uuid, time);
                result.push(time);
            }
        }
        result
        // } else {
        //     let params = [&table, &phone];
        //     let mut stmt = self.conn.prepare(sql).unwrap();
        //     let mut balance_item_row = stmt.query(&params).unwrap();
        //     while let Some(row) = balance_item_row.next().unwrap() {
        //         result.push(row.get(0).unwrap());
        //         result.push(phone);
        //         result.push(row.get(2).unwrap());
        //         result.push(row.get(3).unwrap());
        //         break;
        //     }
        //     return result;
        // }
    }
    pub fn save_data(&mut self, table: String, phone: String, data: Vec<String>) -> bool {
        let mut table_data: Vec<String> = Vec::new();
        if table != "cache" {
            table_data = self.get_table_data(table.clone(), String::from("cache"));
        } else {
            table_data = self.get_table_data(table.clone(), phone.clone());
        }
        let uuid = Uuid::new_v4().to_string();
        match table.as_str() {
            "balance" => {
                if table_data.len() <= 0 {
                    let mut stmt = self.conn.prepare("INSERT INTO balance (uuid,phone,balance,create_time) VALUES (?1,?2,?3,?4)").unwrap();
                    stmt.execute(&[&uuid, &phone, &data[0], &data[1]]).unwrap();
                } else if table_data.len() > 0 {
                    let mut stmt = self
                        .conn
                        .prepare("UPDATE balance SET balance = ?1 WHERE phone = ?2")
                        .unwrap();
                    stmt.execute(&[&data[0], &phone]).unwrap();
                }
            }
            "timer" => {
                if table_data.len() <= 0 {
                    let mut stmt = self
                        .conn
                        .prepare("INSERT INTO timer (uuid,phone,time,money) VALUES (?1,?2,?3,?4)")
                        .unwrap();
                    stmt.execute(&[&uuid, &phone, &data[0], &data[1]]).unwrap();
                }
            }
            "cache" => {
                if table_data.len() <= 0 {
                    let mut stmt = self
                        .conn
                        .prepare("INSERT INTO cache (uuid,time) VALUES (?1,?2)")
                        .unwrap();
                    stmt.execute(&[&uuid, &data[0]]).unwrap();
                }
            }
            _ => {
                return false;
            }
        }
        true
    }
}
