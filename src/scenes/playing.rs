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

pub struct PlayingScene {
    g: *mut crate::game::Game,
}
impl crate::scenes::Scene for PlayingScene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState) {
        if s == ElementState::Pressed && k == KeyCode::P {
            println!("goto menu");
            self.g.ss.pop();
        }
    }
    fn update(&mut self) {
        println!("Main update");
    }
    fn render(&mut self) {}
}