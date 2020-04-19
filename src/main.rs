#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate bytevec;

mod audio;
mod database;
mod game;
mod parsers;
mod scenes;
mod schema;
mod settings;

fn main() -> Result<(), String> {
    let mut g = game::Game::init()?;
    g.load()?;
    while g.playing {
        g.tick()?;
        g.poll()?;
        g.update()?;
        g.render()?;
    }
    Ok(())
}
