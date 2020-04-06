#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod audio;
mod database;
mod parser;

use audio::MusicPlayer;
use database::Database;
use parser::*;

fn main() -> Result<(), String> {
    let db = Database::init()?;

    let osu_parser: Parser<OsuFsm> = Parser::init("maps/osu".into());
    osu_parser.parse_directory(100,10);
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
