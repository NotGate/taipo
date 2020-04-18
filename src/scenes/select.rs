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
pub struct SelectScene {
    // pub maps: Vec<Map>,
    // pub map: Map,
    // pub img: graphics::Image,
    // pub index: usize,
}
impl SelectScene {
    pub fn init(g: &Game) -> Result<SelectScene, String> {
        // // TODO: this is too expensive from a general sense -> request certain chunks at a time (limit+offset)
        // let maps = g.db.query_maps(&g.settings.query)?;
        // let map = maps[0].clone();

        // g.mp.load(&map.audio)?;
        // g.mp.seek(map.preview as f64 / 1000.0)?;
        // g.mp.set_speed(1.2)?;
        // g.mp.set_volume(0.1)?;
        // g.mp.play()?;

        // let img = graphics::Image::new(&mut g.ctx, format!("/{}", map.background.clone()))
        //     .map_err(|e| format!("Could not find img: {}", e))
        //     .unwrap();
        // img.set_filter(graphics::FilterMode::Nearest);

        Ok(SelectScene {
            // g: RefCell::new(Box::new(*g)),
            // maps,
            // map,
            // img,
            // index: 0,
        })
    }
    pub fn enter(g: &mut Game) {
        // let g = self.g.borrow_mut().as_mut();
        // self.img = graphics::Image::new(&mut g.ctx, format!("/{}", self.map.background.clone()))
        //     .map_err(|e| format!("Could not find img: {}", e))
        //     .unwrap();
        // self.img.set_filter(graphics::FilterMode::Nearest);
        // let param = graphics::DrawParam::new()
        //     .dest(nalgebra::Point2::new(0.0, 0.0))
        //     .offset(nalgebra::Point2::new(0.0, 0.0))
        //     .scale(nalgebra::Vector2::new(1.0, 1.0));
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        // let g = self.g.borrow_mut().as_mut();
        // for (e, s, k, m, c) in process(&mut g.el) {
        //     g.ctx.process_event(&e);
        //     if s == ElementState::Pressed {
        //         match k {
        //             KeyCode::Escape => g.playing = false,
        //             KeyCode::Return => {
        //                 g.playing_scene.unwrap().index = 0;
        //                 g.mp.seek(self.map.notes.0[0].0 as f64 / 1000.0 - 1.00)?;
        //                 // g.scene = "Playing".into();
        //                 // g.playing_scene.unwrap().enter();
        //             }
        //             KeyCode::A => g.mp.set_speed(g.mp.get_speed()? - 0.1)?,
        //             KeyCode::D => g.mp.set_speed(g.mp.get_speed()? + 0.1)?,
        //             KeyCode::S => g.mp.set_volume(g.mp.get_volume()? - 0.1)?,
        //             KeyCode::W => g.mp.set_volume(g.mp.get_volume()? + 0.1)?,
        //             KeyCode::H => {
        //                 self.index = (self.index + self.maps.len() - 1) % self.maps.len();
        //                 self.map = self.maps[self.index].clone();
        //                 g.mp.load(&self.map.audio)?;
        //                 println!("{}", self.map.preview);
        //                 g.mp.seek(self.map.preview as f64)?;
        //                 g.mp.set_speed(1.2)?;
        //                 g.mp.set_volume(0.1)?;
        //                 g.mp.play()?;
        //                 self.img = graphics::Image::new(&mut g.ctx, format!("/{}", self.map.background.clone()))
        //                     .map_err(|e| format!("Could not find img: {}", e))
        //                     .unwrap();
        //                 self.img.set_filter(graphics::FilterMode::Nearest);
        //             }
        //             KeyCode::L => {
        //                 self.index = (self.index + self.maps.len() + 1) % self.maps.len();
        //                 self.map = self.maps[self.index].clone();
        //                 g.mp.load(&self.map.audio)?;
        //                 println!("{}", self.map.preview);
        //                 g.mp.seek(self.map.preview as f64)?;
        //                 g.mp.set_speed(1.2)?;
        //                 g.mp.set_volume(0.1)?;
        //                 g.mp.play()?;
        //                 self.img = graphics::Image::new(&mut g.ctx, format!("/{}", self.map.background.clone()))
        //                     .map_err(|e| format!("Could not find img: {}", e))
        //                     .unwrap();
        //                 self.img.set_filter(graphics::FilterMode::Nearest);
        //             }
        //             _ => (),
        //         }
        //     }
        // }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        // println!("main");
        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        // let g = self.g.borrow_mut().as_mut();
        // graphics::draw(
        //     &mut g.ctx,
        //     &self.img,
        //     graphics::DrawParam::new()
        //         .dest(nalgebra::Point2::new(0.0, 0.0))
        //         .offset(nalgebra::Point2::new(0.0, 0.0))
        //         .scale(nalgebra::Vector2::new(1.0, 1.0)),
        // )
        // .unwrap();
        Ok(())
    }
}
