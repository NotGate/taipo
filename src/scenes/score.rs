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

pub struct ScoreScene {
    pub index: usize,
}
impl ScoreScene {
    pub fn init(g: &Game) -> Result<ScoreScene, String> {
        Ok(ScoreScene {
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
