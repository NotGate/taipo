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

use crate::{database::Database, game::Game, scenes::*};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

pub struct PlayingScene {
    //
}
impl PlayingScene {
    pub fn init() -> PlayingScene {
        PlayingScene {}
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => {
                        g.index = 0;
                        g.scene = "Main"
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        if g.mp.pos()? > g.maps[0].notes.0[g.index] as f64 / 1000.0 {
            g.index = (g.index + 1) % g.maps[0].notes.0.len();
        }
        println!("update {}", g.index);
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
}
