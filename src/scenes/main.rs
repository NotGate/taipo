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

pub struct MainScene {
    g: *mut Game,
}
impl MainScene {
    pub fn init(g: *mut Game) -> MainScene {
        MainScene { g }
    }
}
impl Scene for MainScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed {
            match k {
                KeyCode::P => {
                    println!("goto play");
                    // self.g.push(playing::PlayingScene::init(self.g));
                    // unsafe {self.g.as_mut().unwrap().ss.push(Box::new(playing::PlayingScene::init(self.g))) };
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
