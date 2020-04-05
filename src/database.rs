#![allow(dead_code)]

use rusqlite::{params, Connection};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, String> {
        let conn = Connection::open("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}",e))?;
        Ok(Database {
            conn
        })
    }
    pub fn create_table(&self, conn: Connection, table: String, schema: String) -> Result<usize, String> {
        match conn.execute(&format!("CREATE TABLE {} ({})", table, schema), params![]) {
            Ok(count) => Ok(count),
            Err(e) => Err(format!("Could not create table {}: {}", table, e)),
        }
    }
}
