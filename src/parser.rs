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

// osu
pub fn parse(
    directory: &'static str,
    fileglob: &'static str,
    limit: usize,
    batch_size: usize,
    f: fn(&PathBuf) -> Option<Map>,
) {
    glob(
        Path::new(directory)
            .join(fileglob)
            .to_str()
            .expect("Path is invalid unicode"),
    )
    .expect("Invalid glob")
    .filter_map(Result::ok)
    .take(limit)
    .collect::<Vec<PathBuf>>()
    .par_iter()
    .map(f)
    .filter_map(|x| x)
    .collect::<Vec<Map>>()
    .chunks(batch_size)
    .for_each(|chunk| {
        insert(&CONN, &chunk[..]);
    });
}

// sm
// ssc
// bms
// ojn
