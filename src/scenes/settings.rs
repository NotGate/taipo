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

pub struct SettingsScene {
    pub index: usize,
}
impl SettingsScene {
    pub fn init(g: &Game) -> Result<SettingsScene, String> {
        Ok(SettingsScene {
            index: 0,
        })
    }
    pub fn poll(g: &mut Game) -> Result<(),String> {
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(),String> {
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(),String> {
        Ok(())
    }
}
