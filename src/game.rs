use crate::{
    audio::MusicPlayer,
    database::Database,
    parsers::{osu::OsuFsm, parser::Parser},
};
use ggez::{
    event::{
        self,
        winit_event::{Event, KeyboardInput, WindowEvent},
        EventsLoop,
    },
    graphics::{self, DrawMode},
    Context, ContextBuilder,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

// TODO: add an FSM (for renderer in renderers: renderer.render(&mut self))
// TODO: add FPS getting and setting
pub struct Game {
    pub running: bool,
    db: Database,
    mp: MusicPlayer,
    ctx: Rc<RefCell<Context>>,
    el: EventsLoop,
    // Parsers
    osu_p: Parser<OsuFsm>,
}

impl Game {
    pub fn init() -> Result<Game, String> {
        // TODO: look at additional options for all 4 of these
        let (ctx, el) = ContextBuilder::new("eventloop", "ggez")
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

        Ok(Game {
            running: true,
            ctx: Rc::new(RefCell::new(ctx)),
            el,
            db,
            mp,
            osu_p,
        })
    }
    pub fn tick(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(), String> {
        // let thing: std::rc::Rc<EventsLoop> = Rc::new(self.el);
        if let Ok(ctx) = self.ctx.try_borrow_mut().as_mut() {
            self.el.poll_events(|event| {
                ctx.process_event(&event);
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event::quit(ctx),
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        } => match keycode {
                            event::KeyCode::Escape => event::quit(ctx),
                            _ => (),
                        },
                        x => println!("Other window event fired: {:?}", x),
                    },
                    x => println!("Device event fired: {:?}", x),
                }
            });
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), String> {
        // TODO: mp.pos should check if playing
        // println!("{}", self.mp.pos()?);
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), String> {
        if let Ok(ctx) = self.ctx.try_borrow_mut().as_mut() {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            let circle = graphics::Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                nalgebra::Point2::new(0.0, 0.0),
                100.0,
                2.0,
                graphics::WHITE,
            )
            .unwrap();
            graphics::draw(ctx, &circle, (nalgebra::Point2::new(0.0, 380.0),)).unwrap();
            graphics::present(ctx).unwrap();
            ggez::timer::yield_now();
        }
        Ok(())
    }
}
