#![allow(dead_code)]

use std::{hash::{Hash, Hasher},collections::hash_map::DefaultHasher};
use rusqlite::{params, Connection};

#[derive(Hash)]
struct Map {
    
}

// fromto? is that practice specific?
// should the id be a hash?
// mode and median would also be nice to know (mostly mode)
const MAP_SCHEMA: &'static str = "
id              integer primary key,    -- map id
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
difficulty      real,       -- f(count,length,nps,deltas,streaks)
dmin            real,       -- minimum difference between notes (s)
davg            real,       -- average difference between notes (s)
dmax            real,       -- maximum difference between notes (s)
smin            integer,    -- minimum note streak
savg            integer,    -- average note streak
smax            integer,    -- maximum note streak
offsetms        real,       -- audio offset (s)

notes           blob       -- compressed form of [Note]
";

// should I include more than just max combo (I like NF only though)
// an array for error as well as more stats would be nice
const SCORE_SCHEMA: &'static str = "
id              integer primary key,    -- score id
map             integer,                -- map id

score           real,       -- f(map.difficulty,acc,combo,speed,mode)
acc             real,       -- percent accuracy out of 100
combo           integer,    -- max combo
error           integer,    -- average error (s)
speed           real,       -- speed the map was played at (0.5-3.0)
mode            integer,    -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
seed            integer,    -- the random seed
date            integer     -- date the score was achieved
";

const COLLECTION_SCHEMA: &'static str = "
id              integer primary key,    -- collection id
map             integer,                -- map id
name            text                    -- name of collection
";

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, String> {
        let conn = Connection::open("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}",e))?;
        Database::create_table(&conn,"maps",MAP_SCHEMA)?;
        Database::create_table(&conn,"scores",SCORE_SCHEMA)?;
        Database::create_table(&conn,"collection",COLLECTION_SCHEMA)?;
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
