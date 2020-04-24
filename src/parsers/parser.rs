use crate::{database::Database, schema::Map};
use glob::glob;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub struct Parser<T> {
    directory: String,
    t: PhantomData<T>,
}

const LIMIT: usize = 100000;
const BATCH_SIZE: usize = 10000;

impl<T: MapType + Sync> Parser<T> {
    pub fn init(directory: String) -> Self {
        Parser {
            directory,
            t: PhantomData,
        }
    }
    // TODO: should this auto-exectract archives into folders so those can be parsed too?
    pub fn parse_directory(&self, db: &Database) {
        glob(
            Path::new(&self.directory)
                .join(T::glob())
                .to_str()
                .expect("Path is invalid unicode"),
        )
        .expect("Invalid glob")
        .filter_map(Result::ok)
        .take(LIMIT)
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .filter_map(|path| self.parse_file(path))
        .collect::<Vec<Map>>()
        .chunks(BATCH_SIZE)
        .for_each(|chunk| db.insert_maps(&chunk[..]).expect("Could not insert maps chunks"));
    }
    pub fn parse_file(&self, path: &PathBuf) -> Option<Map> {
        let mut fsm = T::init(path);
        BufReader::new(File::open(path).expect(&format!("Could not open {}", path.display())))
            .lines()
            .filter_map(Result::ok)
            .map(|l| String::from(l.trim()))
            .filter(|l| l.len() > 0)
            .for_each(|line| fsm.parse_line(&line));
        fsm.get()
    }
}

pub trait MapType {
    fn init(path: &PathBuf) -> Self;
    fn glob() -> String;
    fn parse_line(&mut self, line: &str);
    fn get(&mut self) -> Option<Map>;
}
