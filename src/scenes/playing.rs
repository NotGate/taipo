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
    pub index: usize,
}
impl PlayingScene {
    pub fn init() -> Result<PlayingScene, String> {
        Ok(PlayingScene { index: 0 })
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => {
                        g.playing_scene.index = 0;
                        g.scene = "Main"
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        // if g.mp.pos()? > g.maps[0].notes.0[g.playing_scene.index] as f64 / 1000.0 {
        //     g.playing_scene.index = (g.playing_scene.index + 1) % g.maps[0].notes.0.len();
        // }

        let vset = 0;
        g.lw = graphics::Text::new("_")
            .set_font(g.font, graphics::Scale::uniform(g.fs))
            .width(&mut g.ctx) as f32;

        let y = (g.h - g.fs) as f64 / 2.0;
        let dx = g.lw as f64 / (g.map.dmin as f64 / 1000.0); //pps
        let travel = (g.w / 2.0 + g.lw) as f64 / dx;   // p / pps = s
        let mut i = g.playing_scene.index;

        // draw a box at w/2 to w/2+lw

        // travel is a second
        // notes - now < travel

        // println!("fs:{} dx:{} travel:{} lw:{} i:{}",g.fs,dx,travel,g.lw,g.playing_scene.index);
        let mut count = 0;
        while i < g.map.notes.0.len() && (g.map.notes.0[i] as f64 / 1000.0 - g.mp.pos()?) < travel as f64 {
            let x = (g.map.notes.0[i] as f64 / 1000.0 - g.mp.pos()?) * dx + (g.w / 2.0) as f64;
            graphics::queue_text(
                &mut g.ctx,
                &graphics::Text::new("a").set_font(g.font, graphics::Scale::uniform(g.fs)),
                nalgebra::Point2::new(x as f32, y as f32),
                None,
            );
            if (g.mp.pos()? - (g.map.notes.0[i] as f64 / 1000.0)) > 0.075 {
                g.playing_scene.index += 1;
            }
            i += 1;
            count += 1;
        }
        println!("{}",count);

        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        let fg = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 400.0, 100.0),
            graphics::Color::new(0.1, 0.2, 0.3, 1.0),
        );
        let bg = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 800.0, 100.0),
            graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        );
        graphics::draw(
            &mut g.ctx,
            &bg.clone().unwrap(),
            (nalgebra::Point2::new(800.0/2.0, (600.0 - 100.0) / 2.0),),
        ).unwrap();
        graphics::draw_queued_text(&mut g.ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Linear).unwrap();
        graphics::draw(
            &mut g.ctx,
            &fg.clone().unwrap(),
            (nalgebra::Point2::new(0.0, (600.0 - 100.0) / 2.0),),
        ).unwrap();
        Ok(())
    }
}
