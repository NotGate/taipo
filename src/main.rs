#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;

mod audio;
mod database;
mod game;
mod parsers;
mod scenes;
mod schema;

fn main() -> Result<(), String> {
    let mut g = game::Game::init()?;
    // g.ss.push(Box::new(scenes::main::MainScene::init(g)));
    while g.playing {
        g.tick()?;
        g.poll()?;
        g.update()?;
        g.render()?;
    }
    Ok(())
}
