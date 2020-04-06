use crate::schema::{Map, COLLECTION_SCHEMA, MAP_SCHEMA, SCORE_SCHEMA};
use rusqlite::{params, Connection};
use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, String> {
        let conn = Connection::open("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}", e))?;
        // nuke tables?
        Database::create_tables(
            &conn,
            &[
                ("maps", MAP_SCHEMA),
                ("scores", SCORE_SCHEMA),
                ("collection", COLLECTION_SCHEMA),
            ],
        )?;
        // verify schema?
        // -- parse/reparse based on taipo.parseDate
        // -- add row to taipo table (remember to preset aset somewhere with mp.get_delay)

        Ok(Database { conn })
    }
    pub fn create_tables(conn: &Connection, tables: &[(&str, &str)]) -> Result<usize, String> {
        tables.iter().fold(Ok(0), |r, (t, s)| Database::create_table(&conn, t, s))
    }
    pub fn create_table(conn: &Connection, table: &str, schema: &str) -> Result<usize, String> {
        conn.execute(&format!("CREATE TABLE IF NOT EXISTS {} ({})", table, schema), params![])
            .map_err(|e| format!("Could not create table {}: {}", table, e))
    }
    // how do I bulk insert??
    pub fn insert_maps(&self, maps: &[Map]) -> Result<(), String> {
        // println!("{:?}", maps);
        let mut stmt = self
            .conn
            .prepare_cached("insert into maps (bpm,nps,difficulty) values (?1,?2,?3)")
            .map_err(|e| format!("Could prepare statement: {}", e))?;
        stmt.execute(&[180.0, 10.5, 30.6])
            .map_err(|e| format!("Could not execute statement: {}", e))?;
        Ok(())
    }
    pub fn exec(&self, stmt: &str) -> Result<(), String> {
        self.conn
            .execute(stmt, params![])
            .map_err(|e| format!("Could not exec statement"))?;
        Ok(())
    }
    pub fn query(&self, stmt: &str) -> Result<Vec<Map>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT bpm,nps,difficulty FROM maps")
            .map_err(|e| format!("Could not exec statement"))?;
        let result = stmt
            .query_map(params![], |row| {
                Ok(Map {
                    bpm: row.get::<usize, f64>(0)? as f32,
                    nps: row.get::<usize, f64>(1)? as f32,
                    difficulty: row.get::<usize, f64>(2)? as f32,
                    ..Default::default()
                })
            })
            .map_err(|e| format!("Could not execute statement: {}", e))?
            .filter_map(Result::ok)
            .collect::<Vec<Map>>();
        Ok(result)
    }

    // FromSql ToSql

    //// bind these to keys or user input
    // exec
    // query

    // insert map(s)
    // insert collection(s)
    // insert score

    // delete map(s)
    // delete collection
    // delete score

    // rename collection (rename to "" = delete?)

    // change taipo settings
}
