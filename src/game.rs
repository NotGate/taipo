use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::Osu, parser::Parser},
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
    ss: Rc<RefCell<SceneStack>>,
    // Parsers
    osu_p: Parser<Osu>,
    // ?
    font: graphics::Font,
    fps_text: graphics::Text,
}

impl Game {
    pub fn init() -> Result<Game, String> {
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
        let ss = Rc::new(RefCell::new(SceneStack::init()));
        ss.borrow_mut().push(MainScene { ss: ss.as_ptr() });

        // Resources (TODO:where do I store all these?)
        let font = graphics::Font::new(&mut ctx, "/fonts/consola.ttf").map_err(|e| format!("Could not find font: {}", e))?;
        let fps_text = graphics::Text::new((ggez::timer::fps(&mut ctx).to_string(), font, 48.0));

        Ok(Game {
            playing: true,
            ctx,
            el,
            db,
            mp,
            ss,
            osu_p,
            font,
            fps_text,
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        for (e, s, k, m) in process(&mut self.el) {
            self.ctx.process_event(&e);
            self.ss.poll(e, s, k, m);
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        // TODO: mp.pos should check if playing
        // println!("{}", self.mp.pos()?);
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

pub trait Scene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState);
    fn update(&mut self);
    fn render(&mut self);
}

pub struct SceneStack {
    stack: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    fn init() -> SceneStack {
        SceneStack { stack: vec![] }
    }
    fn push<T: Scene>(&mut self, scene: T) {}
    fn pop(&mut self) {}
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        self.stack.last().poll(e,s,k,m);
    }
}

// TODO: this shouldn't need to be a raw pointer (probably)
pub struct MainScene {
    ss: *mut SceneStack,
}
pub struct ConfigScene {}
pub struct MapScene {}
pub struct PlayingScene {
    ss: *mut SceneStack,
}
pub struct ScoreScene {}

impl Scene for MainScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed && k == KeyCode::P {
            println!("goto play");
            self.ss.push(PlayingScene { ss:self.ss });
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}
// impl Scene for ConfigScene {}
// impl Scene for MapScene {}
impl Scene for PlayingScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed && k == KeyCode::P {
            println!("goto menu");
            self.ss.pop();
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}
// impl Scene for ScoreScene {}
