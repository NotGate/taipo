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

pub struct PlayingScene {
    g: *mut Game,
}
impl PlayingScene {
    pub fn init(g: *mut Game) -> PlayingScene {
        PlayingScene { g }
    }
}
impl Scene for PlayingScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed && k == KeyCode::P {
            println!("goto menu");
            // self.g.pop();
            // unsafe {self.g.as_mut().unwrap().ss.pop()};
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}
