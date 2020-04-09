pub mod config;
pub mod help;
pub mod main;
pub mod playing;
pub mod score;
pub mod select;

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
use crate::{game::Game};

pub trait Scene {
    fn poll(&mut self, g: &mut Game, e: &Event, s: &ElementState, k: &KeyCode, m: &ModifiersState);
    fn update(&mut self);
    fn render(&mut self);
}
