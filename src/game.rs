use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
    scenes::{config::*, help::*, map::*, playing::*, score::*, Scene},
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

pub struct Game {
    pub playing: bool,
    pub settings: Settings,
    pub ctx: Context,
    pub el: EventsLoop,
    pub db: Database,
    pub mp: MusicPlayer,
    pub osu_p: Parser<Osu>,
    pub scene: Scene,
    pub ms: MapScene,
    pub ps: PlayingScene,
    pub hs: HelpScene,
    pub cs: ConfigScene,
    pub ss: ScoreScene,
}

impl Game {
    pub fn init() -> Result<Self, String> {
        let settings = Settings::init()?;

        let (ctx, el) = ContextBuilder::new("taipo", "notgate")
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

        Ok(Game {
            playing: true,
            settings,
            ctx,
            el,
            db: Database::connect()?,
            mp: MusicPlayer::init()?,
            osu_p: Parser::init("maps/osu".into()), // TODO: async, look for new maps on start
            scene: Scene::Map,
            ms: MapScene::init()?,
            ps: PlayingScene::init()?,
            hs: HelpScene::init()?,
            cs: ConfigScene::init()?,
            ss: ScoreScene::init()?,
        })
    }
    pub fn load(&mut self) -> Result<(), String> {
        self.db.drop_tables()?;
        self.db.create_tables()?;
        self.osu_p.parse_directory(&self.db);

        self.ms.maps = self.db.query_maps(&self.settings.query)?;
        self.ms.map = self.ms.maps[0].clone();

        println!("{}", self.ms.maps.len());
        MapScene::enter(self)
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        match self.scene {
            Scene::Map => MapScene::poll(self)?,
            Scene::Playing => PlayingScene::poll(self)?,
            Scene::Score => ScoreScene::poll(self)?,
            Scene::Help => HelpScene::poll(self)?,
            Scene::Config => ConfigScene::poll(self)?,
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        match self.scene {
            Scene::Map => MapScene::update(self)?,
            Scene::Playing => PlayingScene::update(self)?,
            Scene::Score => ScoreScene::update(self)?,
            Scene::Help => HelpScene::update(self)?,
            Scene::Config => ConfigScene::update(self)?,
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        graphics::clear(&mut self.ctx, [0.1, 0.1, 0.1, 1.0].into());

        match self.scene {
            Scene::Map => MapScene::render(self)?,
            Scene::Playing => PlayingScene::render(self)?,
            Scene::Score => ScoreScene::render(self)?,
            Scene::Help => HelpScene::render(self)?,
            Scene::Config => ConfigScene::render(self)?,
        }

        // let text = graphics::Text::new((
        //     format!("FPS: {}", ggez::timer::fps(&mut self.ctx)),
        //     self.ms.font.unwrap(),
        //     20.0,
        // ));
        // graphics::draw(&mut self.ctx, &text, (nalgebra::Point2::new(0.0, 0.0),)).unwrap();

        graphics::present(&mut self.ctx).unwrap();
        Ok(())
    }
}
