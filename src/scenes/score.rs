use crate::{database::Database, game::Game, scenes::*, schema::Score};
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
use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
    time::Duration,
};

pub struct ScoreScene {
    pub index: usize,
}
impl ScoreScene {
    pub fn init() -> Result<ScoreScene, String> {
        Ok(ScoreScene { index: 0 })
    }
    pub fn enter(g: &mut Game) -> Result<(), String> {
        g.scene = Scene::Score;

        let win = g.settings.window as f32 / 1000.0;

        let abs_error = g.ps.errors.iter().fold(0.0, |acc, e| acc + e.abs()) / g.ps.errors.len() as f32;

        let error = g.ps.errors.iter().fold(0.0, |acc, e| acc + e) / g.ps.errors.len() as f32;
        let acc = g.ps.errors.iter().fold(0.0, |acc, e| acc + ((win - e.abs()) / win)) / g.ps.errors.len() as f32;

        // 1.3 * 100^0.95/100 * 1000 * 13 / 100
        let score = g.settings.speed * (100 as f32).powf(acc) / 100.0 * g.ps.index as f32 * g.ms.map.difficulty
            / g.settings.window as f32;

        let now: DateTime<Utc> = Utc::now();
        let mut s = DefaultHasher::new();
        format!("{}{}", now, g.ms.map.id.clone()).hash(&mut s);
        let id = s.finish().to_string();

        let date: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(now.timestamp() as i64, 0), Utc);
        println!("{}, {}", now.timestamp() as i32, date);

        let score = Score {
            id: "".into(),
            map: g.ms.map.id.clone(),
            score,
            acc,
            error,
            speed: g.settings.speed,
            combo: g.ps.index as i32,
            mode: g.settings.mode.clone(), // this should be a string or enum (4k, typing, etc.)
            seed: g.settings.seed as i32,
            date: now.timestamp() as i32,
        };
        println!("{:?}", score);

        g.db.insert_score(score)?;

        Ok(())
    }
    pub fn poll(g: &mut Game) -> Result<(), String> {
        for (e, s, k, m, c) in process(&mut g.el) {
            g.ctx.process_event(&e);
            if c == '\0' && s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => map::MapScene::enter(g)?,
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
        // TODO: draw score
        // TODO: draw error graph
        Ok(())
    }
}
