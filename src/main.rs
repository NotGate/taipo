#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;

mod audio;
mod database;
mod graphics;
mod input;
mod parsers;
mod schema;

use audio::MusicPlayer;
use database::Database;
use parsers::{osu::OsuFsm, parser::Parser};
use schema::Map;

fn main() -> Result<(), String> {
    let db = Database::connect()?;
    db.drop_tables()?;
    db.create_tables()?;

    let osu_parser: Parser<OsuFsm> = Parser::init("maps/osu".into());
    osu_parser.parse_directory(&db, 100, 10000);

    db.insert_collections(
        "practice",
        &[Map {
            id: "2959750944428650906".into(),
            ..Default::default()
        }],
    )?;
    println!("{:?}", db.query_collections("")?);

    println!("{:?}", db.query_maps("")?);

    db.rename_collection("practice", "fc")?;
    println!("{:?}", db.query_collections("")?);

    db.delete_collection("fc")?;
    println!("{:?}", db.query_collections("")?);

    Ok(())

    // let mut mp = MusicPlayer::init()?;
    // mp.load("assets/sounds/test.mp3")?;
    // mp.set_speed(1.2)?;
    // mp.set_volume(0.6)?;
    // mp.seek(0.0)?;
    // mp.play()?;
    // loop {
    //     println!("{}", mp.pos()?);
    // }
}

// find ./src | entr -cs 'cargo run'

/*
chunk len | time
500       | 6.781s
1000      | 6.161s
5000      | 5.680s
10000     | 5.536s
*/
