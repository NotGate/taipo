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

pub struct MainScene {
    g: *mut crate::game::Game,
}
impl crate::scenes::Scene for MainScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed && k == KeyCode::P {
            println!("goto play");
            self.g.ss.push(crate::scenes::playing::PlayingScene { self.g });
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}