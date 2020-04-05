#![allow(dead_code)]

use rusqlite::{params, Connection};
use std::{
    collections::hash_map::{HashMap,DefaultHasher},
    hash::{Hash, Hasher},
};

pub struct Database {
    conn: Connection,
}

#[derive(Hash)]
struct Map {}

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

// add defaults for everything ...
// fromto? is that practice specific?
// mode and median would also be nice to know (mostly mode)
// query unplayed = where map not in scores
const MAP_SCHEMA: &'static str = r#"
id              integer primary key,    -- map id
source          text,                   -- osu|sm|ssc|bms|ojn
mode            text,                   -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
tags            text,                   -- space separated list of strings
map             integer,                -- hash of map
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

notes           blob       -- compressed form of [Note]?
"#;

// should I include more than just max combo? (I like NF only though)
// an array for error as well as more stats on hit offset would be nice
const SCORE_SCHEMA: &'static str = r#"
id              integer primary key,    -- score id
map             integer,                -- map id

score           real,       -- f(map.difficulty,acc,combo,speed,mode)
acc             real,       -- percent accuracy out of 100
error           real,       -- average error (s)
speed           real,       -- speed the map was played at (0.5-3.0)
combo           integer,    -- max combo
mode            integer,    -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
seed            integer,    -- the random seed
date            integer     -- date the score was achieved
"#;

const COLLECTION_SCHEMA: &'static str = r#"
id              integer primary key,    -- collection id
map             integer,                -- map id
name            text                    -- name of collection
"#;

// font, resolution, window mode, skins, input bindings, etc. (all in db??)
struct Settings {
    // internal settings
    version: String, // taipo version
    query: String,   // last sql query
    parse_date: u64, // date the last map parse was performed (if any folders are newer than that default "", reparse)
    
    // gameplay settings
    mode: String,    // last selected mode (other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k)
    seed: u64,       // last selected seed
    speed: f32,      // last selected speed
    volume: f32,     // last selected volume
    aset: f32, // last selected audio offset (s) - should only ever be negative (play audio sooner) (= -mp.latency() by default)
    iset: f32, // last selected input offset (s) - should only ever be negative (substract from timestamp)
    window: f32, // last selected hit window (s)
    
    // game settings
    skin: String,
    font: String, // Font
    resolution: (f32, f32),
    window_mode: String, // String -> SDL
    bindings: HashMap<String,Vec<u64>>, // u64 -> SDL_Input

}