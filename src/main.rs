#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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

/*
global:
audio offset (notes get there early because audio gets to me late)
-- should only ever be negative (play audio sooner) (= -mp.latency() by default)
input offset (notes are hit late because my input gets to the computer late)
-- should only ever be negative (substract from timestamp)
*/
