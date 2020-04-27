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
    collections::HashMap,
    rc::{Rc, Weak},
    time::Duration,
};

pub struct PlayingScene {
    pub index: usize,
    pub fs: f32,
    pub lw: f32,
    // pub seek: f64,
    pub chars: Vec<char>,
    pub errors: Vec<f32>,
    pub cmap: HashMap<char, graphics::Text>,
    pub fg: Option<graphics::Mesh>,
    pub bg: Option<graphics::Mesh>,
    pub font: Option<graphics::Font>,
    pub text: Option<graphics::Text>,
}

// TODO: split into multiple modes
impl PlayingScene {
    pub fn init() -> Result<PlayingScene, String> {
        Ok(PlayingScene {
            index: 0,
            fs: 0.0,
            lw: 0.0,
            // seek: 0.0,
            chars: vec![],
            errors: vec![],
            cmap: HashMap::new(),
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
        g.ps.errors = vec![];

        // TODO: settings.font
        g.ps.font =
            Some(graphics::Font::new(&mut g.ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?);
        g.ps.text = Some(graphics::Text::new(("_", g.ps.font.unwrap(), g.ps.fs)));

        g.ps.fg = Some(
            graphics::Mesh::new_rectangle(
                &mut g.ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, g.settings.w as f32 / 2.0, g.ps.fs),
                graphics::Color::new(0.1, 0.1, 0.1, 1.0),
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

        let prep = 1.0;
        g.mp.seek((g.ms.map.notes.0[0].0 as f64 / 1000.0 - prep).max(0.0))?;

        // TODO: how to do prep-time now that notes are mapped to audio??
        // g.ps.seek = g.ms.map.notes.0[0].0 as f64 / 1000.0 - prep;
        // if g.ps.seek < 0.0 {
        //     g.mp.mute()?;
        //     g.mp.seek(g.mp.len()?-g.ps.seek)?;
        // } else {
        //     g.mp.seek(g.ps.seek)?;
        // }

        let ascii: Vec<char> = (32u8..127).chain(9..10).chain(13..14).map(|n| n as char).collect();
        let mut rng: StdRng = SeedableRng::seed_from_u64(g.settings.seed as u64);
        g.ps.chars = vec![];
        for note in g.ms.map.notes.0.iter() {
            let ch = *ascii.choose(&mut rng).unwrap();
            g.ps.chars.push(ch);
            g.ps.cmap.insert(
                ch,
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
    // TODO: input timestamp
    // TODO: aset and iset not being used in the right places might be causing weird bugs
    // TODO: print 5 expected (xxaxx) and 3 recieved (xxb)? would need to log presses
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if k == KeyCode::Escape {
                map::MapScene::enter(g)?;
            } else if c != '\0' && c != '\u{1b}' {
                println!("{:?} {:?}",c, g.ps.chars[g.ps.index]);
                let diff = g.mp.pos()? as f32 - g.ms.map.notes.0[g.ps.index].0 as f32 / 1000.0
                    + (g.settings.iset + g.ms.map.offsetms) as f32 / 1000.0;
                if diff.abs() <= g.settings.window as f32 / 1000.0 && c == g.ps.chars[g.ps.index] {
                    g.ps.errors.push(diff);
                    g.ps.index += 1;
                } else {
                    score::ScoreScene::enter(g)?;
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
            let x = (g.ms.map.notes.0[i].0 as f64 / 1000.0 - (g.settings.iset + g.ms.map.offsetms) as f64 / 1000.0 - g.mp.pos()?) * dx + (g.settings.w as f32 / 2.0) as f64;
            graphics::queue_text(
                &mut g.ctx,
                &g.ps.cmap.get(&g.ps.chars[i as usize]).unwrap(),
                nalgebra::Point2::new(x as f32, (g.settings.h as f32 - g.ps.fs) as f32 / 2.0),
                None,
            );
            if (g.mp.pos()? - (g.ms.map.notes.0[i].0 as f64 / 1000.0 - (g.settings.iset + g.ms.map.offsetms) as f64 / 1000.0)) > g.settings.window as f64 / 1000.0 {
                // g.ps.index += 1; // TODO: allow misses for other modes?
                println!("You missed: {:?}",g.ps.chars[i as usize]);
                score::ScoreScene::enter(g)?;
            }
            i += 1;
        }

        Ok(())
    }
    // TODO: take asset creation out of this function
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

        // error
        let n = 10.min(g.ps.errors.len());
        if n == 0 {
            return Ok(());
        }
        let slice = &g.ps.errors[g.ps.errors.len() - n..];
        let mean = slice.iter().fold(0.0, |acc, err| acc + err) / n as f32 * 1000.0;
        let w = g.ps.fs / 40.0;
        let h = g.ps.fs / 5.0;
        let y = (g.settings.h as f32 + g.ps.fs) / 2.0;
        let bar = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, w, h),
            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();
        for err in slice {
            let ratio = *err * 1000.0 / g.settings.window as f32;
            let x = -(ratio * g.ps.lw) + g.settings.w as f32 / 2.0;
            graphics::draw(
                &mut g.ctx,
                &bar,
                (
                    nalgebra::Point2::new(x, y),
                    graphics::Color::new(1.0 * ratio.abs(), 1.0 * (1.0 - ratio.abs()), 0.0, 1.0),
                ),
            )
            .unwrap();
        }
        let x = -(mean / g.settings.window as f32 * g.ps.lw) + g.settings.w as f32 / 2.0;
        graphics::draw(
            &mut g.ctx,
            &graphics::Text::new(format!("{:.0}", mean)),
            (nalgebra::Point2::new(x, y + h),),
        )
        .unwrap();

        Ok(())
    }
}
