#![allow(dead_code)]

use std::{hash::{Hash, Hasher},collections::hash_map::DefaultHasher};
use rusqlite::{params, Connection};

#[derive(Hash)]
struct Map {
    
}

// global:
// audio offset (notes get there early because audio gets to me late)
// -- should only ever be negative (play audio sooner) (= -mp.latency() by default)
// input offset (notes are hit late because my input gets to the computer late)
// -- should only ever be negative (substract from timestamp)

// fromto?
const map_schema: &'static str = "
id              integer primary key,    -- hash of Map
source          text,                   -- osu|sm|ssc|bms|ojn
mode            text,                   -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
tags            text,                   -- space separated list of strings
audio           integer,                -- hash of audio file
background      integer,                -- hash of image file (background offset for osu?)
preview         real,                   -- audio preview (s)

title           text,
artist          text,
creator         text,
version         text,

count           integer,    -- number of notes
length          real,       -- length of song (s)
bpm             real,       -- mode beats per minute
nps             real,       -- avg notes per second
difficulty      real,       -- 
dmin            real,       -- minimum difference between notes (s)
davg            real,       -- average difference between notes (s)
dmax            real,       -- maximum difference between notes (s)
smin            integer,    -- minimum note streak
savg            integer,    -- average note streak
smax            integer,    -- maximum note streak
offsetms        real,       -- audio offset (s)

notes           blob       -- compressed form of [Note]
";

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, String> {
        let conn = Connection::open("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}",e))?;
        Database::create_table(&conn,"maps",map_schema)?;
        // Database::create_table(conn,"scores",scores_schema)?;
        // Database::create_table(conn,"collection",collection_schema)?;
        Ok(Database {
            conn
        })
    }
    pub fn create_table(conn: &Connection, table: &'static str, schema: &'static str) -> Result<usize, String> {
        match conn.execute(&format!("CREATE TABLE IF NOT EXISTS {} ({})", table, schema), params![]) {
            Ok(count) => Ok(count),
            Err(e) => Err(format!("Could not create table {}: {}", table, e)),
        }
    }
}
