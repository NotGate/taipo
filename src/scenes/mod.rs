pub mod main;
pub mod playing;
pub mod config;
pub mod help;
pub mod score;
pub mod select;

pub trait Scene {
    fn poll(&mut self, e: Event, s: ElementState, k: KeyCode, m: ModifiersState);
    fn update(&mut self);
    fn render(&mut self);
}