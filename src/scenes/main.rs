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

pub struct MainScene {
    // pub maps: Vec<Map>,
}
impl MainScene {
    pub fn init() -> Result<MainScene, String> {
        Ok(MainScene {})
    }
    pub fn enter(g: &mut Game) {
        g.img = graphics::Image::new(&mut g.ctx, format!("/{}",g.map.background.clone()))
            .map_err(|e| format!("Could not find img: {}", e))
            .unwrap();
        g.img.set_filter(graphics::FilterMode::Nearest);
        let param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .offset(nalgebra::Point2::new(0.0, 0.0))
            .scale(nalgebra::Vector2::new(1.0, 1.0));
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if s == Some(ElementState::Pressed) && k != None {
                match k.unwrap() {
                    KeyCode::Escape => g.playing = false,
                    KeyCode::Return => {
                        g.playing_scene.index = 0;
                        g.mp.seek(g.map.notes.0[0].0 as f64 / 1000.0 - 1.00)?;
                        g.scene = "Playing";
                        playing::PlayingScene::enter(g);
                    }
                    KeyCode::A => g.mp.set_speed(g.mp.get_speed()? - 0.1)?,
                    KeyCode::D => g.mp.set_speed(g.mp.get_speed()? + 0.1)?,
                    KeyCode::S => g.mp.set_volume(g.mp.get_volume()? - 0.1)?,
                    KeyCode::W => g.mp.set_volume(g.mp.get_volume()? + 0.1)?,
                    KeyCode::H => {
                        g.mi = (g.mi + g.maps.len() - 1) % g.maps.len();
                        g.map = g.maps[g.mi].clone();
                        g.mp.load(&g.map.audio)?;
                        println!("{}",g.map.preview);
                        g.mp.seek(g.map.preview as f64)?;
                        g.mp.set_speed(1.2)?;
                        g.mp.set_volume(0.1)?;
                        g.mp.play()?;
                        g.img = graphics::Image::new(&mut g.ctx, format!("/{}",g.map.background.clone()))
                        .map_err(|e| format!("Could not find img: {}", e))
                        .unwrap();
                    g.img.set_filter(graphics::FilterMode::Nearest);
                    }
                    KeyCode::L => {
                        g.mi = (g.mi + g.maps.len() + 1) % g.maps.len();
                        g.map = g.maps[g.mi].clone();
                        g.mp.load(&g.map.audio)?;
                        println!("{}",g.map.preview);
                        g.mp.seek(g.map.preview as f64)?;
                        g.mp.set_speed(1.2)?;
                        g.mp.set_volume(0.1)?;
                        g.mp.play()?;
                        g.img = graphics::Image::new(&mut g.ctx, format!("/{}",g.map.background.clone()))
                        .map_err(|e| format!("Could not find img: {}", e))
                        .unwrap();
                    g.img.set_filter(graphics::FilterMode::Nearest);
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        // println!("main");
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        graphics::draw(&mut g.ctx, &g.img, g.param).unwrap();
        Ok(())
    }
}
