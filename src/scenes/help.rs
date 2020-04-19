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

pub struct HelpScene {
    pub index: usize,
}
impl HelpScene {
    pub fn init() -> Result<HelpScene, String> {
        Ok(HelpScene { index: 0 })
    }
    pub fn enter(g: &mut Game) -> Result<(), String> {
        g.scene = Scene::Playing;
        g.mp.seek(g.ms.map.notes.0[0].0 as f64 / 1000.0 - 1.00)?;
        Ok(())
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
}
