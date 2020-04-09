use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
    schema::Map,
    // scenes::{main::MainScene, playing::PlayingScene,Scene},
};
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
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Game<'a> {
    pub playing: bool,
    ctx: Context,
    el: EventsLoop,
    //
    db: Database,
    mp: MusicPlayer,
    // Parsers
    osu_p: Parser<Osu>,
    // TODO: this should belong in an overlay?
    font: graphics::Font,
    fps_text: graphics::Text,
    // Scene Stack?
    scene: &'a str,

    // Maps?
    maps: Vec<Map>,
    index: usize,
}

impl<'a> Game<'a> {
    pub fn init() -> Result<Game<'a>, String> {
        // TODO:
        // let settings = Settings::init(); // this will load settings.[filetype]

        // TODO: look at additional options for all 4 of these
        let (mut ctx, el) = ContextBuilder::new("taipo", "notgate")
            .add_resource_path("assets")
            .build()
            .map_err(|e| format!("Could not build ggez context: {}", e))?;

        // Database (TODO: setup)
        let db = Database::connect()?;
        db.drop_tables()?;
        db.create_tables()?;

        // Parser (TODO: scan/add to db)
        let osu_p = Parser::init("maps/osu".into()); // this should come from settings
        osu_p.parse_directory(&db);

        // println!("{:?}", db.query_maps("")?);
        let maps = db.query_maps("")?;

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        // mp.load("assets/sounds/test.mp3")?;
        mp.load(&maps[0].audio)?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.2)?;
        mp.play()?;

        // Resources (TODO:where do I store all these?)
        let font = graphics::Font::new(&mut ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?;
        let fps_text = graphics::Text::new((ggez::timer::fps(&mut ctx).to_string(), font, 48.0));

        Ok(Game {
            playing: true,
            ctx,
            el,
            db,
            mp,
            osu_p,
            font,
            fps_text,
            scene: "Main",
            maps,
            index: 0,
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        match self.scene {
            "Main" => main_poll(self)?,
            "Playing" => playing_poll(self)?,
            _ => (),
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        match self.scene {
            "Main" => main_update(self)?,
            "Playing" => playing_update(self)?,
            _ => (),
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        graphics::clear(&mut self.ctx, [0.1, 0.2, 0.3, 1.0].into());

        match self.scene {
            "Main" => main_render(self)?,
            "Playing" => playing_render(self)?,
            _ => (),
        }

        // fps (temporary)
        self.fps_text = graphics::Text::new((format!("FPS: {}", ggez::timer::fps(&mut self.ctx)), self.font, 48.0));
        graphics::draw(&mut self.ctx, &self.fps_text, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();

        graphics::present(&mut self.ctx).unwrap();
        Ok(())
    }
}

pub fn process(el: &mut EventsLoop) -> Vec<(Event, ElementState, KeyCode, ModifiersState)> {
    let mut events = vec![];
    el.poll_events(|event| match event {
        Event::DeviceEvent {
            event:
                DeviceEvent::Key(KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    modifiers,
                    ..
                }),
            ..
        } => events.push((event, state, key, modifiers)),
        _ => (),
    });
    events
}

pub fn main_poll(g: &mut Game) -> Result<(), String> {
    for (e, s, k, m) in process(&mut g.el) {
        g.ctx.process_event(&e);
        if s == ElementState::Pressed {
            match k {
                KeyCode::Escape => g.playing = false,
                KeyCode::Return => {
                    g.mp.seek(g.maps[0].notes.0[0] as f64 / 1000.0)?;
                    g.scene = "Playing"
                }
                KeyCode::A => g.mp.set_speed(g.mp.get_speed()?-0.1)?,
                KeyCode::D => g.mp.set_speed(g.mp.get_speed()?+0.1)?,
                KeyCode::W => g.mp.set_volume(g.mp.get_volume()?-0.1)?,
                KeyCode::S => g.mp.set_volume(g.mp.get_volume()?+0.1)?,
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn playing_poll(g: &mut Game) -> Result<(), String> {
    for (e, s, k, m) in process(&mut g.el) {
        g.ctx.process_event(&e);
        if s == ElementState::Pressed {
            match k {
                KeyCode::Escape => {
                    g.index = 0;
                    g.scene = "Main"
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn main_update(g: &mut Game) -> Result<(), String> {
    println!("main");
    Ok(())
}

pub fn playing_update(g: &mut Game) -> Result<(), String> {
    // TODO: mp.pos should check if playing
    if g.mp.pos()? > g.maps[0].notes.0[g.index] as f64 / 1000.0 {
        g.index = (g.index + 1) % g.maps[0].notes.0.len();
    }
    println!("update {}", g.index);
    Ok(())
}


pub fn main_render(g: &mut Game) -> Result<(), String> { 
    Ok(())
}

pub fn playing_render(g: &mut Game) -> Result<(), String> { 
    Ok(())
}