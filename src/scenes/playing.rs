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

use crate::{game::Game, database::Database, scenes::*};
use std::{cell::RefCell, rc::{Rc,Weak}, time::Duration};

pub struct PlayingScene {

}
impl PlayingScene {
    pub fn init() -> PlayingScene {
        PlayingScene { }
    }
}
impl Scene for PlayingScene {
    fn poll(&mut self, g: &mut Game, e: &Event, s: &ElementState, k: &KeyCode, m: &ModifiersState) {
        if *s == ElementState::Pressed && *k == KeyCode::P {
            println!("goto menu");
            // g.ss.pop();
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}
