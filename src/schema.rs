// This file is disgusting
// TODO: Please find a way to avoid writing each schema 3 different ways

use diesel::backend::Backend;
use diesel::deserialize::{self, *};
use diesel::serialize::{self, *};
use diesel::sql_types::*;
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Hash)]
#[sql_type = "Binary"]
// TODO: would manually separating a Vec<u64> into two numbers have better performance?
pub struct MapType(pub Vec<(u32, u32)>);
use bytevec::{ByteDecodable, ByteEncodable};
impl<DB: Backend + HasSqlType<Binary>> ToSql<Binary, DB> for MapType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes: &[u8] = &self
            .0
            .encode::<u32>()
            .map_err(|e| format!("Could not convert to sql: {}", e))?;
        <[u8] as ToSql<Binary, DB>>::to_sql(&bytes, out)
    }
}
impl FromSql<Binary, Sqlite> for MapType {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes_vec: Vec<u8> = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        Ok(MapType(
            <Vec<(u32, u32)>>::decode::<u32>(&bytes_vec).map_err(|e| format!("Could not convert from sql: {}", e))?,
        ))
    }
}
impl Default for MapType {
    fn default() -> Self {
        MapType(vec![])
    }
}

// TODO: date_added, last_played
// TODO: top score?
#[derive(Default, Clone, Debug, Insertable, Queryable, QueryableByName)]
#[table_name = "maps"]
pub struct Map {
    pub id: String,
    pub source: String,
    pub mode: String,
    pub format: String,
    pub tags: String,
    pub preview: f32,

    pub map: i32,
    pub audio: String,
    pub background: String,

    pub title: String,
    pub artist: String,
    pub creator: String,
    pub version: String,

    pub keys: i32,
    pub count: i32,
    pub length: f32,
    pub bpm: f32,
    pub nps: f32,
    pub difficulty: f32,
    pub dmin: i32,
    pub davg: i32,
    pub dmax: i32,
    pub smin: i32,
    pub savg: i32,
    pub smax: i32,

    pub offsetms: i32,

    pub notes: MapType,
}

table! {
    maps {
        id -> Text,
        source -> Text,
        mode -> Text,
        format -> Text,
        tags -> Text,
        preview -> Float,

        map -> Integer,
        audio -> Text,
        background -> Text,

        title -> Text,
        artist -> Text,
        creator -> Text,
        version -> Text,

        keys -> Integer,
        count -> Integer,
        length -> Float,
        bpm -> Float,
        nps -> Float,
        difficulty -> Float,
        dmin -> Integer,
        davg -> Integer,
        dmax -> Integer,
        smin -> Integer,
        savg -> Integer,
        smax -> Integer,

        offsetms -> Integer,

        notes -> Blob,
    }
}

// fromto? is that practice specific?
// mode and median would also be nice to know (mostly mode)
// query unplayed = where map not in scores
pub const MAP_SCHEMA: &'static str = r#"
id              text,       -- map id
source          text,       -- osu|sm|ssc|bms|ojn
mode            text,       -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
format          text,       -- file format (v6|v7|v8|..)
tags            text,       -- space separated list of strings
preview         real,       -- audio preview (s)

map             integer,    -- hash of map
audio           text,       -- path (later: hash of audio file)
background      text,       -- path (later: hash of image file (background offset for osu?))

title           text,
artist          text,
creator         text,
version         text,

keys            integer,    -- key count in mania
count           integer,    -- number of notes
length          real,       -- length of song (s)
bpm             real,       -- mode beats per minute
nps             real,       -- avg notes per second
difficulty      real,       -- f(count,length,nps,deltas,streaks)
dmin            integer,    -- minimum difference between notes (ms)
davg            integer,    -- average difference between notes (ms)
dmax            integer,    -- maximum difference between notes (ms)
smin            integer,    -- minimum note streak
savg            integer,    -- average note streak
smax            integer,    -- maximum note streak

offsetms        integer,    -- audio offset (s)

notes           blob        -- compressed form of [Note]?
"#;

// str without chaining lifetimes through everything??
#[derive(Default, Clone, Debug, Insertable, Queryable, QueryableByName)]
#[table_name = "scores"]
pub struct Score {
    pub id: String,
    pub map: String,

    pub score: f32,
    pub acc: f32,
    pub error: f32,
    pub speed: f32,
    pub combo: i32,
    pub mode: i32,
    pub seed: i32,
    pub date: i32,
}

table! {
    scores {
        id -> Text,
        map -> Text,

        score ->           Float,
        acc ->             Float,
        error ->           Float,
        speed ->           Float,
        combo ->           Integer,
        mode ->            Integer,
        seed ->            Integer,
        date ->            Integer,
    }
}

// should I include more than just max combo? (I like NF only though)
// an array for error as well as more stats on hit offset would be nice
pub const SCORE_SCHEMA: &'static str = r#"
id              text,       -- score id
map             text,       -- map id

score           real,       -- f(map.difficulty,acc,combo,speed,mode)
acc             real,       -- percent accuracy out of 100
error           real,       -- average error (s)
speed           real,       -- speed the map was played at (0.5-3.0)
combo           integer,    -- max combo
mode            integer,    -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
seed            integer,    -- the random seed
date            integer     -- date the score was achieved
"#;

#[derive(Default, Clone, Debug, Insertable, Queryable, QueryableByName)]
#[table_name = "collections"]
pub struct Collection {
    pub id: String,
    pub map: String,
    pub name: String,
}

table! {
    collections {
        id -> Text,
        map -> Text,
        name -> Text,
    }
}

pub const COLLECTION_SCHEMA: &'static str = r#"
id              text,    -- collection id
map             text,    -- map id
name            text     -- name of collection
"#;
