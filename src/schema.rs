use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

// str without chaining lifetimes through everything??
#[derive(Debug, Default, Clone)]
pub struct Map {
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
    pub dmin: f32,
    pub davg: f32,
    pub dmax: f32,
    pub smin: i32,
    pub savg: i32,
    pub smax: i32,

    pub offsetms: f32,

    pub notes: Vec<i32>,
}

// fromto? is that practice specific?
// mode and median would also be nice to know (mostly mode)
// query unplayed = where map not in scores
pub const MAP_SCHEMA: &'static str = r#"
id              integer primary key,    -- map id
source          text,                   -- osu|sm|ssc|bms|ojn
mode            text,                   -- other|taiko|1k|2k|3k|4k|5k|6k|7k|8k|9k|10k
format          text,                   -- file format (v6|v7|v8|..)
tags            text,                   -- space separated list of strings
preview         real,                   -- audio preview (s)

map             integer,                -- hash of map
audio           text,                   -- path (later: hash of audio file)
background      text,                   -- path (later: hash of image file (background offset for osu?))

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
pub const SCORE_SCHEMA: &'static str = r#"
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

pub const COLLECTION_SCHEMA: &'static str = r#"
id              integer primary key,    -- collection id
map             integer,                -- map id
name            text                    -- name of collection
"#;

// add defaults for everything
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
