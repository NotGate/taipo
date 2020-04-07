use crate::{audio::MusicPlayer, database::Database, parsers::{parser::Parser,osu::OsuFsm}};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;

// TODO: add an FSM
pub struct Game {
    pub running: bool,
    db: Database,
    mp: MusicPlayer,
    // Graphics
    ctx: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pump: sdl2::EventPump,
    // Parsers
    osu_p: Parser<OsuFsm>
}

impl Game {
    pub fn init() -> Result<Game, String> {
        let ctx = sdl2::init().map_err(|e| format!("Could not initialize SDL2 context: {}", e))?;
        let win = ctx
            .video()
            .map_err(|e| format!("Could not initialize video subsystem: {}", e))?
            .window("taipo", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| format!("Could not build SDL2 window: {}", e))?;
        let canvas = win
            .into_canvas()
            .build()
            .map_err(|e| format!("Could not build SDL2 renderer: {}", e))?;
        let pump = ctx
            .event_pump()
            .map_err(|e| format!("Could not create SDL2 event pump: {}", e))?;
        
        // Database (TODO: setup)
        let db = Database::connect()?;
        
        // Parser (TODO: scan/add to db)
        let osu_p = Parser::init("maps/osu".into());
        osu_p.parse_directory(&db, 20, 10000);

        // Music (TODO: play from db)
        let mut mp = MusicPlayer::init()?;
        mp.load("assets/sounds/test.mp3")?;
        mp.set_speed(1.2)?;
        mp.set_volume(0.6)?;
        mp.play()?;

        Ok(Game {
            running: true,
            db,
            mp,
            ctx,
            canvas,
            pump,
            osu_p
        })
    }
    pub fn tick(&mut self) -> Result<(),String> {
        Ok(())
    }
    pub fn poll(&mut self) -> Result<(),String> {
        for event in self.pump.poll_iter() {
            use sdl2::{event::Event::*, keyboard::Keycode::*};
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(Escape), ..
                } => {
                    self.running = false;
                }
                _ => (),
            }
        }
        Ok(())
    }
    pub fn update(&mut self) -> Result<(),String> {
        println!("{}", self.mp.pos()?);
        Ok(())
    }
    pub fn render(&mut self) -> Result<(),String> {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
        Ok(())
    }
}
