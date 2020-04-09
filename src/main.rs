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

use scenes::{main::MainScene, playing::PlayingScene, Scene};
use std::{cell::RefCell, rc::Rc, time::Duration};

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
