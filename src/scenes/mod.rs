pub mod help;
pub mod playing;
pub mod score;
pub mod select;
pub mod settings;

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

// TODO: change
pub fn process(el: &mut EventsLoop) -> Vec<(Event, ElementState, KeyCode, ModifiersState, char)> {
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
        } => events.push((event, state, key, modifiers, '\0')),
        Event::WindowEvent {
            event: WindowEvent::ReceivedCharacter(c),
            ..
        } => events.push((event, ElementState::Released, KeyCode::G, ModifiersState::default(), c)),
        _ => (),
    });
    events
}
