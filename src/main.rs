#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;

mod audio;
mod database;
mod game;
mod parsers;
mod schema;
mod scenes;

// TODO: this will need to be an FSM
fn main() -> Result<(), String> {
    let mut g = game::Game::init()?;
    while g.playing {
        g.tick()?;
        g.poll()?;
        g.update()?;
        g.render()?;
    }
    Ok(())
}
