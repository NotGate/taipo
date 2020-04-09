use ggez::{
    event::{
        self,
        winit_event::{DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, WindowEvent},
        EventsLoop,
    },
    graphics,
    input::keyboard::KeyCode,
    Context, ContextBuilder,
};

use crate::{game::Game, scenes::*};
use std::{cell::RefCell, rc::{Rc,Weak}, time::Duration};

pub struct MainScene {

}
impl MainScene {
    pub fn init() -> MainScene {
        MainScene { }
    }
}
impl Scene for MainScene {
    fn poll(&mut self, g: &mut Game, e: &Event, s: &ElementState, k: &KeyCode, m: &ModifiersState) {
        if *s == ElementState::Pressed {
            match k {
                KeyCode::P => {
                    println!("goto play");
                    // g.ss.push(Box::new(playing::PlayingScene::init()));
                }
                KeyCode::Escape => {
                    // self.g.playing = false;
                }
                _ => ()
            }
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}
