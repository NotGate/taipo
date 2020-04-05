#![allow(dead_code)]

use rusqlite::{params, Connection};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Hash)]
struct Map {}

// fromto? is that practice specific?
// mode and median would also be nice to know (mostly mode)
// query unplayed = where map not in scores
const MAP_SCHEMA: &'static str = "
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

notes           blob       -- compressed form of [Note]
";

// should I include more than just max combo? (I like NF only though)
// an array for error as well as more stats on hit offset would be nice
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

// do this or use a serialized struct instead?
// font, resolution, window mode, skins, input bindings
const TAIPO_SCHEMA: &'static str = "
version     integer,    -- taipo version
parse       integer,    -- date the last map parse was performed (if any folders are newer than that, reparse)
seed        integer,    -- last selected seed
query       text,       -- last sql query
mode        text,       -- last selected mode (other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k)
speed       real,       -- last selected speed
volume      real,       -- last selected volume
aset        real,       -- last selected audio offset (s) - should only ever be negative (play audio sooner) (= -mp.latency() by default)
iset        real,       -- last selected input offset (s) - should only ever be negative (substract from timestamp)
window      real        -- last selected hit window (s)
";

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Database, String> {
        let conn = Connection::open("taipo.db").map_err(|e| format!("Could not connect to taipo.db: {}", e))?;
        Database::create_tables(
            &conn,
            &[
                ("maps", MAP_SCHEMA),
                ("scores", SCORE_SCHEMA),
                ("collection", COLLECTION_SCHEMA),
            ],
        )?;
        Ok(Database { conn })
    }
    pub fn create_tables(conn: &Connection, tables: &[(&'static str, &'static str)]) -> Result<usize, String> {
        tables
            .iter()
            .fold(Ok(0), |r, (t, s)| Database::create_table(&conn, t, s))
    }
    pub fn create_table(conn: &Connection, table: &'static str, schema: &'static str) -> Result<usize, String> {
        conn.execute(&format!("CREATE TABLE IF NOT EXISTS {} ({})", table, schema), params![])
            .map_err(|e| format!("Could not create table {}: {}", table, e))
    }
}

// #[test]
// fn insert ..

/*
use transactions and caching

conn.execute("INSERT INTO person (name, email) VALUES (?1, ?2)",
&[&name, &email]).unwrap();

conn.execute("INSERT INTO person (name, email) VALUES (:name, :email)",
&[(":name", &name), (":email", &email),])?;

let stmt = self.conn.prepare("INSERT INTO person (name, email) VALUES (:name, :email)")?;

context.conn.execute_batch("BEGIN TRANSACTION;")?;
for p in persons_to_insert {
  context.create_person(&p.name, &p.email)?;
}
context.conn.execute_batch("COMMIT TRANSACTION;")?;



fn insert_new_people(conn: &Connection) -> Result<()> {
    {
        let mut stmt = conn.prepare_cached("INSERT INTO People (name) VALUES (?)")?;
        stmt.execute(&["Joe Smith"])?;
    }
    {
        // This will return the same underlying SQLite statement handle without
        // having to prepare it again.
        let mut stmt = conn.prepare_cached("INSERT INTO People (name) VALUES (?)")?;
        stmt.execute(&["Bob Jones"])?;
    }
    Ok(())
}

fn insert_new_people(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO People (name) VALUES (?)")?;
    stmt.execute(&["Joe Smith"])?;
    stmt.execute(&["Bob Jones"])?;
    Ok(())
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "BEGIN;
                        CREATE TABLE foo(x INTEGER);
                        CREATE TABLE bar(y TEXT);
                        COMMIT;",
    )
}

fn update_rows(conn: &Connection) {
    match conn.execute("UPDATE foo SET bar = 'baz' WHERE qux = ?", &[1i32]) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("update failed: {}", err),
    }
}
*/
