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
    pub fn enter(g: &mut Game) {
        let fg = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 400.0, 100.0),
            graphics::Color::new(0.1, 0.2, 0.3, 1.0),
        )
        .unwrap();
        let bg = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, g.w, 100.0),
            graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        )
        .unwrap();

        // or load this from file, stdin, etc.
        g.chars = vec![];
        for note in g.map.notes.0.iter() {
            g.chars.push(
                graphics::Text::new("a")
                    .set_font(g.font, graphics::Scale::uniform(g.fs))
                    .to_owned(),
            );
        }

        g.lw = graphics::Text::new("_")
            .set_font(g.font, graphics::Scale::uniform(g.fs))
            .width(&mut g.ctx) as f32;
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if k == Some(KeyCode::Escape) {
                g.scene = "Main";
            } else if c != None {
                println!("{} {}", c.unwrap(), g.chars[g.playing_scene.index].contents().pop().unwrap());
                if c.unwrap() == g.chars[g.playing_scene.index].contents().pop().unwrap() {
                    println!(
                        "good :) {}",
                        g.map.notes.0[g.playing_scene.index].0 as f64 / 1000.0 - g.mp.pos().unwrap() + 0.06
                    );
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {

        // Typing
        let dx = g.lw as f64 / (g.map.dmin as f64 / 1000.0);
        let travel = (g.w / 2.0 + g.lw) as f64 / dx;
        let mut i = g.playing_scene.index;
        while i < g.map.notes.0.len() && (g.map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) < travel as f64 {
            let x = (g.map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) * dx + (g.w / 2.0) as f64;
            graphics::queue_text(
                &mut g.ctx,
                &g.chars[i as usize],
                nalgebra::Point2::new(x as f32, (g.h - g.fs) as f32 / 2.0),
                None,
            );
            if (g.mp.pos()? - (g.map.notes.0[i].0 as f64 / 1000.0)) > 0.075 {
                g.playing_scene.index += 1;
            }
            i += 1;
        }

        // Mania
        

        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        graphics::draw(&mut g.ctx, &g.bg, (nalgebra::Point2::new(g.w / 2.0, (g.h - g.fs) / 2.0),)).unwrap();
        graphics::draw_queued_text(&mut g.ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Linear).unwrap();
        graphics::draw(&mut g.ctx, &g.fg, (nalgebra::Point2::new(0.0, (g.h - g.fs) / 2.0),)).unwrap();
        Ok(())
    }
}
