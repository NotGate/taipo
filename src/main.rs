#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod audio;
mod database;
mod graphics;
mod input;
mod parsers;
mod schema;

use audio::MusicPlayer;
use database::Database;
use parsers::{osu::OsuFsm, parser::Parser};

fn main() -> Result<(), String> {
    let db = Database::init()?;

    let osu_parser: Parser<OsuFsm> = Parser::init("maps/osu".into());
    db.exec("delete from maps")?;
    osu_parser.parse_directory(&db, 20, 1000);
    println!("{:?}", db.query("")?);

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
10        | 5.707s
50        | 4.299s
100       | 4.321s
500       | 4.215s
1000      | 4.153s
5000      | 4.315s
10000     | 4.197s
*/
