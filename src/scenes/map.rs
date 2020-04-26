use chrono::{DateTime, NaiveDateTime, Utc};
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
use tabwriter::TabWriter;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use crate::{game::Game, scenes::*, schema::Map};
use std::{
    io::Write,
    cell::RefCell,
    f32,
    rc::{Rc, Weak},
    time::Duration,
};

const FONT_SIZE: f32 = 20.0;

pub struct MapScene {
    pub index: usize,
    pub maps: Vec<Map>,
    pub map: Map,
    pub font: Option<graphics::Font>,
    pub ctext: Option<graphics::Text>,
    pub mtext: Option<graphics::Text>,
    pub stext: Option<graphics::Text>,
    pub bg: Option<graphics::Image>,
}
impl MapScene {
    pub fn init() -> Result<MapScene, String> {
        Ok(MapScene {
            index: 0,
            maps: vec![],
            map: Map::default(),
            font: None,
            ctext: None,
            mtext: None,
            stext: None,
            bg: None,
        })
    }
    pub fn enter(g: &mut Game) -> Result<(), String> {
        g.scene = Scene::Map;
        // TODO: settings.font
        g.ms.font =
            Some(graphics::Font::new(&mut g.ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?);
        MapScene::change_map(g)?;
        Ok(())
    }
    // TODO: reduce repitition
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if c == '\0' && s == ElementState::Pressed {
                let amt = if m.alt { 5 } else { 1 } * if m.shift { -1 } else { 1 };
                // used: QSNVAIWORJF
                //
                match k {
                    KeyCode::Q => g.playing = false,
                    KeyCode::Escape => config::ConfigScene::enter(g)?,
                    KeyCode::Return => playing::PlayingScene::enter(g)?,
                    KeyCode::Slash => help::HelpScene::enter(g)?, // TODO: question mark instead of slash
                    KeyCode::Space => {
                        if g.mp.is_playing()? {
                            g.mp.pause()?
                        } else {
                            g.mp.play()?
                        }
                    }
                    KeyCode::N => {
                        g.ms.index = MapScene::wrap(g.ms.index as i32, amt, g.ms.maps.len() as i32);
                        MapScene::change_map(g)?;
                    }
                    KeyCode::S => {
                        g.mp.set_speed(g.mp.get_speed()? + amt as f32 / 100.0)?;
                        g.settings.speed = g.mp.get_speed()?;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::V => {
                        g.mp.set_volume(g.mp.get_volume()? + amt as f32 / 100.0)?;
                        g.settings.volume = g.mp.get_volume()?;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::A => {
                        g.settings.aset = num::clamp(g.settings.aset + amt, -10000, 10000);
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::I => {
                        g.settings.iset = num::clamp(g.settings.iset + amt, -10000, 10000);
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::W => {
                        g.settings.window = num::clamp(g.settings.window + amt, -10000, 10000);
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::O => {
                        g.ms.map.offsetms = num::clamp(g.ms.map.offsetms + amt, -10000, 10000);
                        g.ms.maps[g.ms.index].offsetms = g.ms.map.offsetms;
                        g.db.update_map_offset(&g.ms.map)?;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::R => {
                        let mut rng: StdRng = SeedableRng::seed_from_u64(g.settings.seed as u64);
                        g.settings.seed = rng.gen_range(1000, 10000) as u64;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::J => {
                        let mut rng: StdRng = SeedableRng::seed_from_u64(g.settings.seed + g.ms.index as u64);
                        g.ms.index = rng.gen_range(0, g.ms.maps.len());
                        MapScene::change_map(g)?;
                    }
                    KeyCode::F => {
                        // focus query input
                    }
                    KeyCode::M => {
                        // TODO: toggle modes
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
    // TODO: the information is there, just make it prettier and cleaner
    // TODO: remove asset creation into a load function instead of creating them every frame
    pub fn render(g: &mut Game) -> Result<(), String> {
        // draw Maps[Image]
        if let Some(bg) = g.ms.bg.as_ref() {
            graphics::draw(
                &mut g.ctx,
                bg,
                graphics::DrawParam::new()
                    .dest(nalgebra::Point2::new(g.settings.w as f32 * 2.0 / 3.0, 0.0))
                    .offset(nalgebra::Point2::new(0.0, 0.0))
                    .scale(nalgebra::Vector2::new(
                        g.settings.w as f32 / bg.width() as f32 / 3.0,
                        g.settings.h as f32 / bg.height() as f32 / 3.0,
                    )),
            )
            .unwrap();
        }

        let mut y = 0.0;

        // draw query
        let text_box = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, g.settings.w as f32, FONT_SIZE),
            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();
        graphics::draw(&mut g.ctx, &text_box, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();
        let mut text = graphics::Text::new(g.settings.query.clone());
        text.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(FONT_SIZE))
            .set_bounds(
                nalgebra::Point2::new(g.settings.w as f32, f32::INFINITY),
                graphics::Align::Left,
            );
        graphics::draw(
            &mut g.ctx,
            &text,
            graphics::DrawParam::new().dest(nalgebra::Point2::new(0.0, y)).color(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
        )
        .unwrap();
        y += FONT_SIZE;

        // draw Settings{}
        if let Some(ctext) = g.ms.ctext.as_ref() {
            graphics::draw(&mut g.ctx, ctext, (nalgebra::Point2::new(0.0, y),)).unwrap();
            y += ctext.height(&mut g.ctx) as f32 + FONT_SIZE
        }

        // draw Map{}
        // draw diff color bg?
        if let Some(mtext) = g.ms.mtext.as_ref() {
            graphics::draw(&mut g.ctx, mtext, (nalgebra::Point2::new(0.0, y),)).unwrap();
            y += mtext.height(&mut g.ctx) as f32 + FONT_SIZE
        }

        // draw graph
        let dt = g.ms.map.dmin as f32;
        let t = g.ms.map.notes.0.last().unwrap().0 as f32;
        let dx = dt / t * g.settings.w as f32 * 0.66;
        let wr = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, dx, FONT_SIZE),
            graphics::WHITE,
        )
        .unwrap();
        let cursor = graphics::Mesh::new_rectangle(
            &mut g.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 10.0, FONT_SIZE),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
        for note in g.ms.map.notes.0.iter() {
            graphics::draw(
                &mut g.ctx,
                &wr,
                (nalgebra::Point2::new(note.0 as f32 / t * g.settings.w as f32 * 0.66, y),),
            )
            .unwrap();
        }
        graphics::draw(
            &mut g.ctx,
            &cursor,
            (nalgebra::Point2::new(
                g.mp.pos()? as f32 * 1000.0 / t * g.settings.w as f32 * 0.66,
                y,
            ),),
        )
        .unwrap();
        y += 40.0;

        // draw Scores[{}]
        if let Some(stext) = g.ms.stext.as_ref() {
            graphics::draw(&mut g.ctx, stext, (nalgebra::Point2::new(0.0, y),)).unwrap();
            // y += stext.height(&mut g.ctx) as f32 + FONT_SIZE
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
        println!("{:?}", g.ms.map);
        // println!("\n{}\n", g.ms.map.preview);
        if g.ms.map.audio != *old_audio || !g.mp.is_playing()? {
            g.mp.load(&g.ms.map.audio)?;
            g.mp.seek(g.ms.map.preview as f64)?;
            g.mp.set_speed(g.settings.speed)?;
            g.mp.set_volume(g.settings.volume)?;
            g.mp.play()?;
        }
        MapScene::update_ctext(g)?;
        MapScene::update_mtext(g)?;
        MapScene::update_stext(g)?;
        MapScene::update_bg(g)
    }
    fn update_mtext(g: &mut Game) -> Result<(), String> {
        g.ms.mtext = Some(graphics::Text::new(format!(
            "{} - {} [{}]
Mapped by {}
Mode:{} Keys:{} Length:{} Count:{} BPM:{:.2}
Difficulty:{:.2} NPS:{:.2} Delta:[{},{},{}] Streak:[{},{},{}]",
            g.ms.map.artist,
            g.ms.map.title,
            g.ms.map.version,
            g.ms.map.creator,
            g.ms.map.mode,
            g.ms.map.keys,
            (g.ms.map.length / g.settings.speed) as i32, // Duration::milliseconds(map.length as i64).format("%H:%M:%S"),
            g.ms.map.count,
            g.ms.map.bpm * g.settings.speed,
            g.ms.map.difficulty * g.settings.speed,
            g.ms.map.nps * g.settings.speed,
            (g.ms.map.dmin as f32 / g.settings.speed) as i32,
            (g.ms.map.davg as f32 / g.settings.speed) as i32,
            (g.ms.map.dmax as f32 / g.settings.speed) as i32,
            g.ms.map.smin,
            g.ms.map.savg,
            g.ms.map.smax,
        )));
        if let Some(v) = g.ms.mtext.as_mut() {
            v.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(FONT_SIZE))
                .set_bounds(
                    nalgebra::Point2::new(g.settings.w as f32, f32::INFINITY),
                    graphics::Align::Left,
                );
        }
        Ok(())
    }
    fn update_ctext(g: &mut Game) -> Result<(), String> {
        g.ms.ctext = Some(graphics::Text::new(format!(
            "Collection:{}
Map:{}/{}
Speed:{:.2} Volume:{:.2} Mode:{} Seed:{}
aset:{} iset:{} window:{} local:{}",
            "", //g.collections[g.c_i],
            g.ms.index + 1,
            g.ms.maps.len(),
            g.settings.speed,
            g.settings.volume,
            g.settings.mode,
            g.settings.seed,
            g.settings.aset,
            g.settings.iset,
            g.settings.window,
            g.ms.map.offsetms,
        )));
        if let Some(v) = g.ms.ctext.as_mut() {
            v.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(FONT_SIZE))
                .set_bounds(
                    nalgebra::Point2::new(g.settings.w as f32, f32::INFINITY),
                    graphics::Align::Left,
                );
        }
        Ok(())
    }
    fn update_stext(g: &mut Game) -> Result<(), String> {
        println!("{}",g.ms.map.id);
        let scores = g.db.query_scores(&format!("map={}", g.ms.map.id))?;
        println!("scores: {:?}",scores);
        let mut tw = TabWriter::new(vec![]);
        write!(&mut tw, "Score\tAcc\tError\tCombo\tSpeed\tDate\tMode\tSeed\n")
            .map_err(|e| format!("Couldn't write tabwriter header: {}", e))?;
        for score in scores {
            write!(
                &mut tw,
                "{:.2}\t{:.2}\t{:.2}\t{}\t{:.2}\t{}\t{}\t{}\n",
                score.score,
                score.acc * 100.0,
                score.error * 1000.0,
                score.combo,
                score.speed,
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(score.date as i64, 0), Utc)
                    .format("%a %d/%m/%y %I:%M %p"), // Day D/M/Y H:M
                score.mode,
                score.seed
            )
            .map_err(|e| format!("Couldn't write to tabwriter: {}", e))?;
        }
        tw.flush().map_err(|e| format!("Couldn't flush tabwriter: {}", e))?;
        g.ms.stext = Some(graphics::Text::new(
            String::from_utf8(
                tw.into_inner()
                    .map_err(|e| format!("Couldn't convert tabwriter to writer: {}", e))?,
            )
            .map_err(|e| format!("Couldn't convert tabwriter to string: {}", e))?,
        ));
        if let Some(v) = g.ms.stext.as_mut() {
            v.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(FONT_SIZE))
                .set_bounds(
                    nalgebra::Point2::new(g.settings.w as f32, f32::INFINITY),
                    graphics::Align::Left,
                );
        }
        Ok(())
    }
}
