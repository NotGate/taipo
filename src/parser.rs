#![allow(unused_imports)]

use glob::glob;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct Map {}

const CONN: i32 = 0;
fn insert(c: &i32, chunk: &[Map]) {
    println!("{:?}", chunk);
}


struct Parser<T:FSM+Default> {
    fsm: T, // don't need this as a field (remember things are multithreaded)
    // parse_directory
    // parse_file
    // -- let mut m = Map::default()
    // -- let mut fsm = T::default()
    // -- fsm.load(&mut m);
    // -- fsm.parse_line("fldkj");
}

// should this auto-exectract archives into folders so those can be parsed too?
pub fn parse_directory(directory: &str, fileglob: &str, limit: usize, batch_size: usize, f: fn(&PathBuf) -> Option<Map>) {
    glob(Path::new(directory).join(fileglob).to_str().expect("Path is invalid unicode"))
        .expect("Invalid glob")
        .filter_map(Result::ok)
        .take(limit)
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .filter_map(f)
        .collect::<Vec<Map>>()
        .chunks(batch_size)
        .for_each(|chunk| insert(&CONN, &chunk[..]));
}

pub fn parse_file<T:FSM>(path: &PathBuf, fsm:&mut T) -> Result<Map, String> {
    let m = Map::default();
    // fsm.load_map(&mut m);
    BufReader::new(File::open(path).map_err(|e| format!("Couldn't open file {}: {}", path.display(), e))?)
        .lines()
        .filter_map(Result::ok)
        //.map(|l| l.trim())
        .for_each(|line| fsm.parse_line(&line));
    Ok(m)
}

// FSM { osu { start, general, ..., end }, bms, sm, ojn }
// parse -> change &state, change &map

pub trait FSM {
    fn parse_line(&mut self,line:&str);

}

#[derive(Debug)]
pub enum OSU {
    Start,
    General,      //kv
    Metadata,     //kv
    Editor,       //kv
    Difficulty,   //kv
    Events,       //Comma-separated lists
    TimingPoints, //Comma-separated lists
    Colours,      //kv
    HitObjects,   //Comma-separated lists
    End
}

impl FSM for OSU {
    fn parse_line(&mut self,line:&str) {
        *self = match std::mem::replace(self, OSU::Start) {
            OSU::Start => OSU::General,
            OSU::General => OSU::Metadata,
            OSU::Metadata => OSU::Editor,
            OSU::Editor => OSU::Difficulty,
            OSU::Difficulty => OSU::Events,
            OSU::Events => OSU::TimingPoints,
            OSU::TimingPoints => OSU::Colours,
            OSU::Colours => OSU::HitObjects,
            OSU::HitObjects => OSU::End,
            OSU::End => OSU::End,
        };
        println!("{:?}",self);
    }
}

pub enum SM {}
pub enum SSC {}
pub enum BMS {}
pub enum OJN {}