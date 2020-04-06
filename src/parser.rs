#![allow(unused_imports)]

use glob::glob;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Map {
    nps: f32
}

const CONN: i32 = 0;
fn insert(c: &i32, chunk: &[Map]) {
    println!("{:?}", chunk);
}

use std::marker::PhantomData;
pub struct Parser<T> {
    directory: String,
    t: PhantomData<T>,
}

impl<T: FSM + Sync> Parser<T> {
    pub fn init(directory: String) -> Self {
        Parser {
            directory,
            t: PhantomData,
        }
    }
    // should this auto-exectract archives into folders so those can be parsed too?
    pub fn parse_directory(&self, limit: usize, batch_size: usize) {
        glob(
            Path::new(&self.directory)
                .join(T::glob())
                .to_str()
                .expect("Path is invalid unicode"),
        )
        .expect("Invalid glob")
        .filter_map(Result::ok)
        .take(limit)
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .filter_map(|path| self.parse_file(path))
        .collect::<Vec<Map>>()
        .chunks(batch_size)
        .for_each(|chunk| insert(&CONN, &chunk[..]));
    }
    pub fn parse_file(&self, path: &PathBuf) -> Option<Map> {
        let mut fsm = T::init();
        BufReader::new(File::open(path).expect(&format!("Could not open {}", path.display())))
            .lines()
            .filter_map(Result::ok)
            .for_each(|line| fsm.parse_line(&line));
        Some(fsm.get())
    }
}

pub trait FSM {
    fn init() -> Self;
    fn glob() -> String;
    fn parse_line(&mut self, line: &str);
    fn get(&self) -> Map;
}

#[derive(Debug)]
pub struct OsuFsm {
    map: Map,
    state: OsuState,
}

#[derive(Debug,std::cmp::PartialEq)]
pub enum OsuState {
    Start,
    General,      //kv
    Metadata,     //kv
    Editor,       //kv
    Difficulty,   //kv
    Events,       //Comma-separated lists
    TimingPoints, //Comma-separated lists
    Colours,      //kv
    HitObjects,   //Comma-separated lists
    End,
}

impl FSM for OsuFsm {
    fn init() -> Self {
        OsuFsm {
            map: Map::default(),
            state: OsuState::Start,
        }
    }
    fn glob() -> String {
        "**/*.osu".into()
    }
    fn parse_line(&mut self, line: &str) {
        use OsuState::*;
        if self.state == Start {
            self.map.nps = 3.0;
        }
        self.state = match self.state {
            Start => General,
            General => Metadata,
            Metadata => Editor,
            Editor => Difficulty,
            Difficulty => Events,
            Events => TimingPoints,
            TimingPoints => Colours,
            Colours => HitObjects,
            HitObjects => End,
            End => End,
        };
        if self.state != End {
            println!("{:?},{}", self.state,line);
        }
    }
    fn get(&self) -> Map {
        self.map.clone()
    }
}

pub enum SM {}
pub enum SSC {}
pub enum BMS {}
pub enum OJN {}

// type Thunk = Box<dyn Fn() + Send + 'static>;
