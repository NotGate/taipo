#![allow(dead_code)]

mod audio;
mod database;
use audio::MusicPlayer;
use database::Database;
fn main() -> Result<(), String> {
    let db = Database::init()?;
    Ok(())
}

/*
let mut mp = MusicPlayer::init()?;
mp.load("assets/sounds/test.mp3")?;
mp.set_speed(1.2)?;
mp.set_volume(0.6)?;
mp.seek(0.0)?;
mp.play()?;
loop {
    println!("{}", mp.pos()?);
}
*/

// find ./src | entr -cs 'cargo run'
