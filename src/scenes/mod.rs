pub mod config;
pub mod help;
pub mod main;
pub mod playing;
pub mod score;
pub mod select;

use crate::game::Game;
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

pub fn process(el: &mut EventsLoop) -> Vec<(Event, ElementState, KeyCode, ModifiersState)> {
    let mut events = vec![];
    el.poll_events(|event| match event {
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            modifiers,
                            ..
                        },
                    ..
                },
            ..
        } => events.push((event, state, key, modifiers)),
        _ => (),
    });
    events
}

/*
// TODO: this should belong in an overlay?
pub font: graphics::Font,
pub fps_text: graphics::Text,
// Resources (TODO:where do I store all these?)
// they should be in their respective Scene/Overlay
// TODO: fonts should be selectable from the system?
// TODO: font size should be changable and come from Settings
let font = graphics::Font::new(&mut ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?;
let fps_text = graphics::Text::new((ggez::timer::fps(&mut ctx).to_string(), font, 48.0));
self.fps_text = graphics::Text::new((format!("FPS: {}", ggez::timer::fps(&mut self.ctx)), self.font, 48.0));
graphics::draw(&mut self.ctx, &self.fps_text, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();
*/
