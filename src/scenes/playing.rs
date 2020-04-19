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
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

pub struct PlayingScene {
    pub index: usize,
    pub fs: f32,
    pub lw: f32,
    pub chars: Vec<graphics::Text>,
    pub fg: Option<graphics::Mesh>,
    pub bg: Option<graphics::Mesh>,
    pub font: Option<graphics::Font>,
    pub text: Option<graphics::Text>,
}

impl PlayingScene {
    pub fn init() -> Result<PlayingScene, String> {
        Ok(PlayingScene {
            index: 0,
            fs: 0.0,
            lw: 0.0,
            chars: vec![],
            fg: None,
            bg: None,
            font: None,
            text: None,
        })
    }
    pub fn enter(g: &mut Game) -> Result<(), String> {
        g.scene = Scene::Playing;
        g.ps.index = 0;
        g.ps.fs = 150.0; //g.settings.fs;

        // TODO: settings.font
        g.ps.font =
            Some(graphics::Font::new(&mut g.ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?);
        g.ps.text = Some(graphics::Text::new(("_", g.ps.font.unwrap(), g.ps.fs)));

        g.ps.fg = Some(
            graphics::Mesh::new_rectangle(
                &mut g.ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, g.settings.w as f32 / 2.0, g.ps.fs),
                graphics::Color::new(0.1, 0.2, 0.3, 1.0),
            )
            .unwrap(),
        );
        g.ps.bg = Some(
            graphics::Mesh::new_rectangle(
                &mut g.ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, g.settings.w as f32 / 2.0, g.ps.fs),
                graphics::Color::new(0.2, 0.2, 0.2, 1.0),
            )
            .unwrap(),
        );

        g.ps.lw = graphics::Text::new("_")
            .set_font(g.ps.font.unwrap(), graphics::Scale::uniform(g.ps.fs))
            .width(&mut g.ctx) as f32;
        g.mp.seek(g.ms.map.notes.0[0].0 as f64 / 1000.0 - 1.00)?;

        let ascii: Vec<char> = (32u8..127).chain(9..10).chain(13..14).map(|n| n as char).collect();
        let mut rng: StdRng = SeedableRng::seed_from_u64(g.settings.seed as u64);
        g.ps.chars = vec![];
        for note in g.ms.map.notes.0.iter() {
            let ch = *ascii.choose(&mut rng).unwrap();
            g.ps.chars.push(
                graphics::Text::new(graphics::TextFragment {
                    text: if ch == ' ' || ch == '\r' || ch == '\t' {
                        '\u{263B}'
                    } else {
                        ch
                    }
                    .to_string(),
                    color: Some(match ch {
                        ' ' => graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                        '\r' => graphics::Color::new(1.0, 1.0, 0.0, 1.0),
                        '\t' => graphics::Color::new(1.0, 0.0, 1.0, 1.0),
                        _ => graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                    }),
                    font: Some(g.ps.font.unwrap()),
                    scale: Some(graphics::Scale::uniform(g.ps.fs)),
                })
                .to_owned(),
            );
        }
        Ok(())
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if k == KeyCode::Escape {
                map::MapScene::enter(g)?;
            } else {
                println!("{} {}", c, g.ps.chars[g.ps.index].contents().pop().unwrap());
                if c == g.ps.chars[g.ps.index].contents().pop().unwrap() {
                    println!(
                        "good :) {}",
                        g.ms.map.notes.0[g.ps.index].0 as f64 / 1000.0 - g.mp.pos().unwrap() + 0.06
                    );
                }
            }
        }
        Ok(())
    }
    pub fn update(g: &mut Game) -> Result<(), String> {
        let dx = g.ps.lw as f64 / (g.ms.map.dmin as f64 / 1000.0);
        let travel = (g.settings.w as f32 / 2.0 + g.ps.lw) as f64 / dx;
        let mut i = g.ps.index;
        while i < g.ms.map.notes.0.len() && (g.ms.map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) < travel as f64 {
            let x = (g.ms.map.notes.0[i].0 as f64 / 1000.0 - g.mp.pos()?) * dx + (g.settings.w as f32 / 2.0) as f64;
            graphics::queue_text(
                &mut g.ctx,
                &g.ps.chars[i as usize],
                nalgebra::Point2::new(x as f32, (g.settings.h as f32 - g.ps.fs) as f32 / 2.0),
                None,
            );
            if (g.mp.pos()? - (g.ms.map.notes.0[i].0 as f64 / 1000.0)) > g.settings.window as f64 / 1000.0 {
                g.ps.index += 1;
            }
            i += 1;
        }

        Ok(())
    }
    pub fn render(g: &mut Game) -> Result<(), String> {
        graphics::draw(
            &mut g.ctx,
            &g.ps.bg.clone().unwrap(),
            (nalgebra::Point2::new(
                g.settings.w as f32 / 2.0,
                (g.settings.h as f32 - g.ps.fs) / 2.0,
            ),),
        )
        .unwrap();
        graphics::draw_queued_text(&mut g.ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Linear).unwrap();
        graphics::draw(
            &mut g.ctx,
            &g.ps.fg.clone().unwrap(),
            (nalgebra::Point2::new(0.0, (g.settings.h as f32 - g.ps.fs) / 2.0),),
        )
        .unwrap();

        // TODO: draw error

        Ok(())
    }
}
