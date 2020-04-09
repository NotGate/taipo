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
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

pub struct MainScene {
    //
}
impl MainScene {
    pub fn init() -> MainScene {
        MainScene {}
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => g.playing = false,
                    KeyCode::Return => {
                        g.mp.seek(g.maps[0].notes.0[0] as f64 / 1000.0)?;
                        g.scene = "Playing"
                    }
                    KeyCode::A => g.mp.set_speed(g.mp.get_speed()? - 0.1)?,
                    KeyCode::D => g.mp.set_speed(g.mp.get_speed()? + 0.1)?,
                    KeyCode::W => g.mp.set_volume(g.mp.get_volume()? - 0.1)?,
                    KeyCode::S => g.mp.set_volume(g.mp.get_volume()? + 0.1)?,
                    _ => (),
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        println!("main");
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
}
