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

use crate::{game::Game, scenes::*, schema::Map};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};
pub struct MapScene {
    pub maps: Vec<Map>,
    pub map: Map,
    pub index: usize,
    pub bg: Option<graphics::Image>,
}
impl MapScene {
    pub fn init() -> Result<MapScene, String> {
        Ok(MapScene {
            maps: vec![],
            map: Map::default(),
            bg: None,
            index: 0,
        })
    }
    pub fn enter(g: &mut Game) -> Result<(), String> {
        g.scene = Scene::Map;
        MapScene::change_map(g)?;
        // g.mp.seek(g.ms.map.preview as f64 / 1000.0)?;
        Ok(())
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if c == '\0' && s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => g.playing = false,
                    KeyCode::Return => playing::PlayingScene::enter(g)?,
                    KeyCode::A => g.mp.set_speed(g.mp.get_speed()? - 0.1)?,
                    KeyCode::D => g.mp.set_speed(g.mp.get_speed()? + 0.1)?,
                    KeyCode::S => g.mp.set_volume(g.mp.get_volume()? - 0.1)?,
                    KeyCode::W => g.mp.set_volume(g.mp.get_volume()? + 0.1)?,
                    KeyCode::H => {
                        g.ms.index = MapScene::wrap(g.ms.index as i32, -1, g.ms.maps.len() as i32);
                        MapScene::change_map(g)?;
                    }
                    KeyCode::L => {
                        g.ms.index = MapScene::wrap(g.ms.index as i32, 1, g.ms.maps.len() as i32);
                        MapScene::change_map(g)?;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        if g.ms.bg.is_some() {
            graphics::draw(
                &mut g.ctx,
                &g.ms.bg.clone().unwrap(),
                graphics::DrawParam::new()
                    .dest(nalgebra::Point2::new(0.0, 0.0))
                    .offset(nalgebra::Point2::new(0.0, 0.0))
                    .scale(nalgebra::Vector2::new(1.0, 1.0)),
            )
            .unwrap();
        }
        Ok(())
    }

    // Private Helper Functions
    fn update_bg(g: &mut Game) -> Result<(), String> {
        if let Ok(bg) =
            graphics::Image::new(&mut g.ctx, format!("/{}", g.ms.map.background)).map_err(|e| format!("Could not find bg: {}", e))
        {
            g.ms.bg = Some(bg);
            g.ms.bg.as_mut().map(|v| v.set_filter(graphics::FilterMode::Nearest));
        } else {
            g.ms.bg = None;
        }
        Ok(())
    }
    fn wrap(start: i32, amt: i32, len: i32) -> usize {
        ((start + len + amt) % len) as usize
    }
    fn change_map(g: &mut Game) -> Result<(), String> {
        let old_audio = &g.ms.map.audio.clone();
        g.ms.map = g.ms.maps[g.ms.index].clone();
        if g.ms.map.audio != *old_audio || !g.mp.is_playing()? {
            g.mp.load(&g.ms.map.audio)?;
            g.mp.seek(g.ms.map.preview as f64)?;
            g.mp.set_speed(g.settings.speed)?;
            g.mp.set_volume(g.settings.volume)?;
            g.mp.play()?;
        }
        MapScene::update_bg(g)
    }
}
