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

use std::{cell::RefCell, rc::Rc, time::Duration};
use     scenes::{main::MainScene, playing::PlayingScene,Scene};

fn main() -> Result<(), String> {
    let mut g = game::Game::init()?;
    // g.ss.push(Box::new(scenes::main::MainScene::init()));
    // g.ss.push(Box::new(scenes::playing::PlayingScene::init()));

    // let mut main = scenes::main::MainScene::init();
    // let mut playing = scenes::playing::PlayingScene::init();
    // match g.scene.as_str() {
    //     "Main" => main.poll(&mut g, &e, &s, &k, &m),
    //     "Playing" => playing.poll(&mut g, &e, &s, &k, &m),
    //     _ => ()
    // }
    while g.playing {
        g.tick()?;
        g.poll()?;
        g.update()?;
        g.render()?;
    }
    Ok(())
}
