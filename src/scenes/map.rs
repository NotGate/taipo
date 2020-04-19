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
    f32,
    rc::{Rc, Weak},
    time::Duration,
};
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
        g.ms.font =
            Some(graphics::Font::new(&mut g.ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?);
        MapScene::change_map(g)?;
        // g.mp.seek(g.ms.map.preview as f64 / 1000.0)?;
        Ok(())
    }
    // TODO: reduce repitition
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if c == '\0' && s == ElementState::Pressed {
                use ggez::event::KeyCode;
                match k {
                    KeyCode::Q => g.playing = false, // TODO: proper closing (save settings)
                    KeyCode::Escape => config::ConfigScene::enter(g)?,
                    KeyCode::Return => playing::PlayingScene::enter(g)?,
                    KeyCode::Slash => help::HelpScene::enter(g)?,
                    // Speed
                    KeyCode::A => {
                        g.mp.set_speed(g.mp.get_speed()? - 0.1)?;
                        g.settings.speed = g.mp.get_speed()?;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::D => {
                        g.mp.set_speed(g.mp.get_speed()? + 0.1)?;
                        g.settings.speed = g.mp.get_speed()?;
                        MapScene::update_ctext(g)?;
                    }
                    // Volume
                    KeyCode::S => {
                        g.mp.set_volume(g.mp.get_volume()? - 0.1)?;
                        g.settings.volume = g.mp.get_volume()?;
                        MapScene::update_ctext(g)?;
                    }
                    KeyCode::W => {
                        g.mp.set_volume(g.mp.get_volume()? + 0.1)?;
                        g.settings.volume = g.mp.get_volume()?;
                        MapScene::update_ctext(g)?;
                    }
                    // Index
                    KeyCode::H => {
                        g.ms.index = MapScene::wrap(g.ms.index as i32, -1, g.ms.maps.len() as i32);
                        MapScene::change_map(g)?;
                    }
                    KeyCode::L => {
                        g.ms.index = MapScene::wrap(g.ms.index as i32, 1, g.ms.maps.len() as i32);
                        MapScene::change_map(g)?;
                    }
                    KeyCode::R => {
                        use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
                        let mut rng: StdRng = SeedableRng::seed_from_u64(g.settings.seed as u64);
                        g.settings.seed = rng.gen_range(1000, 10000) as u64;
                        MapScene::update_ctext(g)?;
                    }
                    // aset+-
                    // iset+-
                    // window+-
                    // offset+-
                    // mode
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
        // draw Settings{}
        if let Some(ctext) = g.ms.ctext.as_ref() {
            graphics::draw(&mut g.ctx, ctext, (nalgebra::Point2::new(0.0,0.0),)).unwrap();
        }
        // draw Map{}
        // draw diff color bg?
        if let Some(mtext) = g.ms.mtext.as_ref() {
            graphics::draw(&mut g.ctx, mtext, (nalgebra::Point2::new(0.0, g.settings.h as f32 / 3.0),)).unwrap();
        }
        // draw Scores[{}]
        if let Some(stext) = g.ms.stext.as_ref() {
            graphics::draw(&mut g.ctx, stext, (nalgebra::Point2::new(0.0, g.settings.h as f32 * 2.0 / 3.0),)).unwrap();
        }
        // draw Maps[Image]
        if let Some(bg) = g.ms.bg.as_ref() {
            graphics::draw(
                &mut g.ctx,
                bg,
                graphics::DrawParam::new()
                    .dest(nalgebra::Point2::new(g.settings.w as f32 / 2.0, g.settings.h as f32 / 4.0))
                    .offset(nalgebra::Point2::new(0.0, 0.0))
                    .scale(nalgebra::Vector2::new(
                        g.settings.w as f32 / bg.width() as f32 / 2.0,
                        g.settings.h as f32 / bg.height() as f32 / 2.0,
                    )),
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
        MapScene::update_ctext(g)?;
        MapScene::update_mtext(g)?;
        MapScene::update_bg(g)
    }
    fn update_mtext(g: &mut Game) -> Result<(), String> {
        g.ms.mtext = Some(graphics::Text::new(format!(
            "{} - {} [{}] ({})
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
            v.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(15.0)).set_bounds(
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
            v.set_font(g.ms.font.unwrap(), graphics::Scale::uniform(15.0)).set_bounds(
                nalgebra::Point2::new(g.settings.w as f32, f32::INFINITY),
                graphics::Align::Left,
            );
        }
        Ok(())
    }
}
