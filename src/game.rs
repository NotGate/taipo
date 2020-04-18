use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
    scenes::{help::*, playing::*, score::*, select::*, settings::*},
    schema::Map,
    settings::Settings,
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

#[derive(Clone,Copy)]
pub enum Scene {
    Select,
    Playing,
    Score,
    Help,
    Settings,
}

pub struct Game {
    pub playing: bool,
    pub settings: Settings,
    pub ctx: Context,
    pub el: EventsLoop,
    pub db: Database,
    pub mp: MusicPlayer,
    pub osu_p: Parser<Osu>,
    pub scene: Scene,
    pub select_scene: Option<SelectScene>,
    pub playing_scene: Option<PlayingScene>,
    pub help_scene: Option<HelpScene>,
    pub settings_scene: Option<SettingsScene>,
    pub score_scene: Option<ScoreScene>,
}

impl Game {
    pub fn init() -> Result<Self, String> {
        let settings = Settings::init()?;

        let (mut ctx, el) = ContextBuilder::new("taipo", "notgate")
            .window_mode(
                WindowMode::default()
                    .dimensions(settings.w as f32, settings.h as f32)
                    .borderless(settings.borderless)
                    .maximized(settings.maximized)
                    .fullscreen_type(ggez::conf::FullscreenType::Windowed)
                    .resizable(false),
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

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;

        Ok(Game {
            playing: true,
            settings,
            ctx,
            el,
            db,
            mp,
            osu_p,
            scene: Scene::Select,
            select_scene: None,
            playing_scene: None,
            help_scene: None,
            settings_scene: None,
            score_scene: None,
        })
    }
    // gross hack because Rust constructors are annoying
    pub fn load(&mut self) -> Result<(), String> {
        self.select_scene = Some(SelectScene::init(self)?);
        self.playing_scene = Some(PlayingScene::init(self)?);
        self.help_scene = Some(HelpScene::init(self)?);
        self.settings_scene = Some(SettingsScene::init(self)?);
        self.score_scene = Some(ScoreScene::init(self)?);
        Ok(())
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        match self.scene {
            Scene::Select => SelectScene::poll(self)?,
            Scene::Playing => PlayingScene::poll(self)?,
            Scene::Score => ScoreScene::poll(self)?,
            Scene::Help =>HelpScene::poll(self)?,
            Scene::Settings => SettingsScene::poll(self)?,
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        match self.scene {
            Scene::Select => SelectScene::update(self)?,
            Scene::Playing => PlayingScene::update(self)?,
            Scene::Score => ScoreScene::update(self)?,
            Scene::Help =>HelpScene::update(self)?,
            Scene::Settings => SettingsScene::update(self)?,
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        graphics::clear(&mut self.ctx, [0.1, 0.2, 0.3, 1.0].into());

        match self.scene {
            Scene::Select => SelectScene::poll(self)?,
            Scene::Playing => PlayingScene::poll(self)?,
            Scene::Score => ScoreScene::poll(self)?,
            Scene::Help =>HelpScene::poll(self)?,
            Scene::Settings => SettingsScene::poll(self)?,
        }

        graphics::present(&mut self.ctx).unwrap();
        Ok(())
    }
}
