use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::OsuFsm, parser::Parser},
};
use ggez::{
    event::{
        self,
        winit_event::{DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, WindowEvent},
        EventsLoop,
    },
    input::keyboard::KeyCode,
    graphics, Context, ContextBuilder};
use std::time::Duration;

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
    osu_p: Parser<OsuFsm>,
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
        osu_p.parse_directory(&db, 20, 10000); // define these as global constants

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        mp.load("assets/sounds/test.mp3")?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.6)?;
        mp.play()?;

        // Graphics ?
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
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        self.ctx.timer_context.tick();
        ggez::timer::yield_now();
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        use ggez::event::winit_event::ElementState;
        use ggez::input::keyboard::KeyCode;
        for (e, s, k, m) in process(&mut self.el) {
            self.ctx.process_event(&e);
            if s == ElementState::Pressed {
                match k {
                    KeyCode::Escape => self.playing = false,
                    _ => (),
                }
                println!("{:?}", k);
            }
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
