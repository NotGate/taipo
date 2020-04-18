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
    // pub index: usize,
    // pub fg: graphics::Mesh,
    // pub bg: graphics::Mesh,
    // pub fs: f32,
    // pub lw: f32,
    // pub font: graphics::Font,
    // pub text: graphics::Text,

    // pub chars: Vec<graphics::Text>,
}
impl PlayingScene {
    pub fn init(g: &Game) -> Result<PlayingScene, String> {
        // let fs = 100.0;
        // let lw = 0.0;

        // // TODO: settings.font
        // let font = graphics::Font::new(&mut g.ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?;
        // let text = graphics::Text::new(("_", font, fs));

        // let fg = graphics::Mesh::new_rectangle(
        //     &mut g.ctx,
        //     graphics::DrawMode::fill(),
        //     graphics::Rect::new(0.0, 0.0, 400.0, 100.0),
        //     graphics::Color::new(0.1, 0.2, 0.3, 1.0),
        // )
        // .unwrap();
        // let bg = graphics::Mesh::new_rectangle(
        //     &mut g.ctx,
        //     graphics::DrawMode::fill(),
        //     graphics::Rect::new(0.0, 0.0, g.settings.w as f32 as f32, 100.0),
        //     graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        // )
        // .unwrap();

        Ok(PlayingScene {
            // g: RefCell::new(Box::new(*g)),
            // index: 0,
            // fg,
            // bg,
            // fs,
            // lw,
            // font,
            // text,
            // chars: vec![]
        })
    }
    pub fn enter(g: &mut Game) {
    //     let g = self.g.borrow_mut().as_mut();

    //     // or load this from file, stdin, etc.
    //     self.chars = vec![];
    //     for note in g.select_scene.unwrap().map.notes.0.iter() {
    //         self.chars.push(
    //             graphics::Text::new("a")
    //                 .set_font(self.font, graphics::Scale::uniform(self.fs))
    //                 .to_owned(),
    //         );
    //     }

    //     self.lw = graphics::Text::new("_")
    //         .set_font(self.font, graphics::Scale::uniform(self.fs))
    //         .width(&mut g.ctx) as f32;
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
    //     let g = self.g.borrow_mut().as_mut();
    //     for (e, s, k, m, c) in process(&mut g.el) {
    //         g.ctx.process_event(&e);
    //         if k == KeyCode::Escape {
    //             // g.scene = "Main".into();
    //         } else {
    //             println!("{} {}", c, self.chars[self.index].contents().pop().unwrap());
    //             if c == self.chars[self.index].contents().pop().unwrap() {
    //                 println!(
    //                     "good :) {}",
    //                     g.select_scene.unwrap().map.notes.0[self.index].0 as f64 / 1000.0 - g.mp.pos().unwrap() + 0.06
    //                 );
    //             }
    //         }
    //     }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        // let g = self.g.borrow_mut().as_mut();
        // // Typing
        // let dx = self.lw as f64 / (g.select_scene.unwrap().map.dmin as f64 / 1000.0);
        // let travel = (g.settings.w as f32 / 2.0 + self.lw) as f64 / dx;
        // let mut i = self.index;
        // while i < g.select_scene.unwrap().map.notes.0.len() && (g.select_scene.unwrap().map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) < travel as f64 {
        //     let x = (g.select_scene.unwrap().map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) * dx + (g.settings.w as f32 / 2.0) as f64;
        //     graphics::queue_text(
        //         &mut g.ctx,
        //         &self.chars[i as usize],
        //         nalgebra::Point2::new(x as f32, (g.settings.h as f32 - self.fs) as f32 / 2.0),
        //         None,
        //     );
        //     if (g.mp.pos()? - (g.select_scene.unwrap().map.notes.0[i].0 as f64 / 1000.0)) > 0.075 {
        //         self.index += 1;
        //     }
        //     i += 1;
        // }

        // Mania

        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        // let g = self.g.borrow_mut().as_mut();
        // graphics::draw(&mut g.ctx, &self.bg, (nalgebra::Point2::new(g.settings.w as f32 / 2.0, (g.settings.h as f32 - self.fs) / 2.0),)).unwrap();
        // graphics::draw_queued_text(&mut g.ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Linear).unwrap();
        // graphics::draw(&mut g.ctx, &self.fg, (nalgebra::Point2::new(0.0, (g.settings.h as f32 - self.fs) / 2.0),)).unwrap();
        Ok(())
    }
}
