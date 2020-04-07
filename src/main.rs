#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;

mod game;
mod audio;
mod database;
mod parsers;
mod schema;

use game::Game;

fn main() -> Result<(), String> {
    let mut g = Game::init()?;
    while g.running {
        g.tick()?;
        g.poll()?;
        g.update()?;
        g.render()?;
    }
    Ok(())
}