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

impl<T: FSM + Sync> Parser<T> {
    pub fn init(directory: String) -> Self {
        Parser {
            directory,
            t: PhantomData,
        }
    }
    // should this auto-exectract archives into folders so those can be parsed too?
    pub fn parse_directory(&self, db: &Database, limit: usize, batch_size: usize) {
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
        .for_each(|chunk| db.insert_maps(&chunk[..]));
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
