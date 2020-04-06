// This file is disgusting
// Please find a way to avoid writing each schema 3 different ways

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

// str without chaining lifetimes through everything??
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

    pub notes: String,
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

        notes -> Text,
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

notes           text        -- compressed form of [Note]?
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

// PLEASE REDUCE REPTITION SOMEHOW
// #![feature(concat_idents)]
// macro_rules! t {
//     ($name:ident $($i:ident:$t:ty)*) => {
//         #[derive(Default, Clone, Debug, Queryable, Insertable)]
//         #[table_name="apples"]
//         pub struct $name {
//             pub $i: $t,
//         }
//         table! {
//             $name {

//             }
//         }
//     };
// }

// t!{Apple
//     id:i32
//     source:String
//     preview:f32
// }

// add defaults for everything
// remember to preset aset somewhere with mp.get_delay
pub struct Settings {
    // internal settings
    version: String, // taipo version
    query: String,   // last sql query
    parse_date: u64, // date the last map parse was performed (if any folders are newer than that default "", reparse)

    // gameplay settings
    mode: String, // last selected mode (other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k)
    seed: u64,    // last selected seed
    speed: f32,   // last selected speed
    volume: f32,  // last selected volume
    aset: f32, // last selected audio offset (s) - should only ever be negative (play audio sooner) (= -mp.latency() by default)
    iset: f32, // last selected input offset (s) - should only ever be negative (substract from timestamp)
    window: f32, // last selected hit window (s)

    // game settings
    skin: String,
    font: String, // Font
    resolution: (f32, f32),
    window_mode: String,                 // String -> SDL
    bindings: HashMap<String, Vec<u64>>, // u64 -> SDL_Input
}
