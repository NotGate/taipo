use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
    scenes::{main::*, playing::*},
    schema::Map,
};
use ggez::{
    conf::WindowMode,
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
    pub db: Database,
    pub mp: MusicPlayer,
    pub osu_p: Parser<Osu>,
    // TODO: Scene Stack?
    // SceneManager?
    pub scene: &'a str,
    pub main_scene: MainScene,
    pub playing_scene: PlayingScene,
    // TODO: these shouldn't be in Game
    // Some also shouldn't be in Scenes because they should be in Settings
    pub maps: Vec<Map>,
    pub chars: Vec<graphics::Text>, // just store glyphs?
    pub map: Map,
    pub w: f32,
    pub h: f32,
    pub fs: f32,
    pub lw: f32,
    pub mi: usize,
    pub text: graphics::Text,
    pub font: graphics::Font,
    pub fps_text: graphics::Text,
    pub bg: graphics::Mesh,
    pub fg: graphics::Mesh,
    pub img: graphics::Image,
    pub param: graphics::DrawParam,
}

impl<'a> Game<'a> {
    pub fn init() -> Result<Game<'a>, String> {
        // TODO:
        // let settings = Settings::init(); // this will load settings.[filetype]

        let w = 800.0;
        let h = 600.0;
        let fs = 100.0;

        let (mut ctx, el) = ContextBuilder::new("taipo", "notgate")
            .window_mode(
                WindowMode::default()
                    .dimensions(w, h)
                    .borderless(true)
                    .maximized(false)
                    .fullscreen_type(ggez::conf::FullscreenType::Windowed),
                // .resizable(false),
            )
            .add_resource_path("assets")
            .add_resource_path(".")
            .build()
            .map_err(|e| format!("Could not build ggez context: {}", e))?;

        // Database (TODO: setup)
        let db = Database::connect()?;
        db.drop_tables()?;
        db.create_tables()?;

        // Parser (TODO: looks for `new` maps on start and add them)
        // TODO: parse async so it doesn't block the game
        let osu_p = Parser::init("maps/osu".into()); // this should come from settings
        osu_p.parse_directory(&db);

        // TODO: this is too expensive from a general sense -> request certain chunks at a time (limit+offset)
        let maps = db.query_maps("smin>30 and dmin between 50 and 100")?;
        let map = maps[0].clone();

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        mp.load(&map.audio)?;
        mp.seek(map.preview as f64 / 1000.0)?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.1)?;
        mp.play()?;

        let font = graphics::Font::new(&mut ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?;
        let text = graphics::Text::new(("_", font, fs));
        let fps_text = graphics::Text::new((ggez::timer::fps(&mut ctx).to_string(), font, 48.0));
        let fg = graphics::Mesh::new_rectangle(
            &mut ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 400.0, 100.0),
            graphics::Color::new(0.1, 0.2, 0.3, 1.0),
        )
        .unwrap();
        let bg = graphics::Mesh::new_rectangle(
            &mut ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 800.0, 100.0),
            graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        )
        .unwrap();

        let mut img = graphics::Image::new(&mut ctx, format!("/{}",map.background.clone()))
            .map_err(|e| format!("Could not find img: {}", e))
            .unwrap();
        img.set_filter(graphics::FilterMode::Nearest);
        let param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .offset(nalgebra::Point2::new(0.0, 0.0))
            .scale(nalgebra::Vector2::new(1.0, 1.0));

        Ok(Game {
            playing: true,
            ctx,
            el,
            db,
            mp,
            osu_p,
            scene: "Main",
            main_scene: MainScene::init()?,
            playing_scene: PlayingScene::init()?,
            maps,
            map,
            mi: 0,
            font,
            text,
            fps_text,
            w,
            h,
            fs,
            lw: 0.0,
            fg,
            bg,
            chars: vec![],
            img,
            param,
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        // self.scene.poll();
        match self.scene {
            "Main" => MainScene::poll(self)?,
            "Playing" => PlayingScene::poll(self)?,
            _ => (),
        }
        Ok(())
    }
    // self.scene.update();
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
        // self.scene.render();
        match self.scene {
            "Main" => MainScene::render(self)?,
            "Playing" => PlayingScene::render(self)?,
            _ => (),
        }

        self.fps_text = graphics::Text::new((format!("FPS: {}", ggez::timer::fps(&mut self.ctx)), self.font, 10.0));
        graphics::draw(&mut self.ctx, &self.fps_text, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();

        graphics::present(&mut self.ctx).unwrap();
        Ok(())
    }
}
