use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
    scenes::{main::*, playing::*},
    schema::Map,
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
    pub ctx: Context,
    pub el: EventsLoop,
    //
    pub db: Database,
    pub mp: MusicPlayer,
    // Parsers
    pub osu_p: Parser<Osu>,

    // TODO: Scene Stack? MainScene::update()
    pub scene: &'a str,

    // Maps?
    pub maps: Vec<Map>,
    pub index: usize,

    // TODO: this should belong in an overlay?
    pub font: graphics::Font,
    pub fps_text: graphics::Text,
}

impl<'a> Game<'a> {
    pub fn init() -> Result<Game<'a>, String> {
        // TODO:
        // let settings = Settings::init(); // this will load settings.[filetype]

        let (mut ctx, el) = ContextBuilder::new("taipo", "notgate")
            .add_resource_path("assets")
            .build()
            .map_err(|e| format!("Could not build ggez context: {}", e))?;

        // Database (TODO: setup)
        let db = Database::connect()?;
        db.drop_tables()?;
        db.create_tables()?;

        // Parser (TODO: looks for `new` maps on start and add them)
        let osu_p = Parser::init("maps/osu".into()); // this should come from settings
                                                     // TODO: parse async so it doesn't block the game
        osu_p.parse_directory(&db);

        // TODO: this is too expensive from a general sense -> request certain chunks at a time (limit+offset)
        let maps = db.query_maps("")?;

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        mp.load(&maps[0].audio)?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.2)?;
        mp.play()?;

        // Resources (TODO:where do I store all these?)
        // they should be in their respective Scene/Overlay
        // TODO: fonts should be selectable from the system?
        // TODO: font size should be changable and come from Settings
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
            "Main" => MainScene::poll(self)?,
            "Playing" => PlayingScene::poll(self)?,
            _ => (),
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        match self.scene {
            "Main" => MainScene::update(self)?,
            "Playing" => PlayingScene::update(self)?,
            _ => (),
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        graphics::clear(&mut self.ctx, [0.1, 0.2, 0.3, 1.0].into());

        // TODO: each scene should have a list of overlays to render but so should main?
        match self.scene {
            "Main" => MainScene::render(self)?,
            "Playing" => PlayingScene::render(self)?,
            _ => (),
        }

        // fps (temporary)
        self.fps_text = graphics::Text::new((format!("FPS: {}", ggez::timer::fps(&mut self.ctx)), self.font, 48.0));
        graphics::draw(&mut self.ctx, &self.fps_text, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();

        graphics::present(&mut self.ctx).unwrap();
        Ok(())
    }
}
