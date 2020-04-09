use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
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

// TODO: add an FSM (for renderer in renderers: renderer.render(&mut self))
// TODO: add FPS getting and setting
pub struct Game {
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
    // Scene Stack
    scene: String,
    playing_scene: Scene,
    main_scene: Scene,
}

struct Main {}
struct Settings {}
struct Help {}
struct Select {}
struct Playing {}
struct Score {}

enum Scene {
    None,
    Main(Main),
    Settings(Settings),
    Help(Help),
    Select(Select),
    Playing(Playing),
    Score(Score)
}

impl Game {
    pub fn init() -> Result<Game, String> {
        // TODO: 
        // let settings = Settings::init(); // this will load settings.[filetype]

        // TODO: look at additional options for all 4 of these
        let (mut ctx, el) = ContextBuilder::new("taipo", "notgate")
            .add_resource_path("assets")
            .build()
            .map_err(|e| format!("Could not build ggez context: {}", e))?;

        // Database (TODO: setup)
        let db = Database::connect()?;

        // Parser (TODO: scan/add to db)
        let osu_p = Parser::init("maps/osu".into()); // this should come from settings
        osu_p.parse_directory(&db);

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        mp.load("assets/sounds/test.mp3")?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.6)?;
        mp.play()?;

        // SceneStack
        let main_scene = Scene::Main(Main {});
        let playing_scene = Scene::Playing(Playing {});
        let scene =String::from("Main");

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
            scene,
            main_scene,
            playing_scene
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {

        // Systems iterating over objects instead of giving my Scene their own functions
        // working with their data
        // I am implementing scenes is just using enumerators and match statement. Different scenes will run different systems and will work with different components

        for (e, s, k, m) in process(&mut self.el) {
            match self.scene.as_str() {
                "Main" => {
                    f(self);
                }
                "Playing" => {
                    g(self);
                },
                _ => (),
            }
            // self.ctx.process_event(&e);
            // use std::borrow::BorrowMut;
            // use std::boxed::*;
            // use std::any::Any;
            // // let thing:Box<dyn Scene> = self.ss[0].downcast::<MainScene>();
            // let thing:Box<dyn Any> = self.ss[0];
            // if let Ok(string) = thing.downcast::<MainScene>() {
            //     println!("String ({}): {}", string.len(), string);
            // }
            // match self.scene.as_str() {
            //     "Playing" => self.ss[0].poll(self, &e,&s,&k,&m),
            //     _ => self.scene = "Main".into()
            // }
            // self.ss[0].poll(&mut self.db, &e,&s,&k,&m);
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        // TODO: mp.pos should check if playing
        // println!("{}", self.mp.pos()?);
        match self.scene.as_str() {
            "Main" => {
                f(self);
            }
            "Playing" => {
                g(self);
            },
            _ => (),
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        graphics::clear(&mut self.ctx, [0.1, 0.2, 0.3, 1.0].into());
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

pub fn f(g:&mut Game) {
    println!("Playing");
    g.scene = String::from("Playing");
}

pub fn g(g:&mut Game) {
    println!("Main");
    g.scene = String::from("Main");
}