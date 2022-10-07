use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use crate::window_manager::{Frame, WindowEvents};

mod game;
pub use game::Game;

pub trait WindowController {
    fn on_init(&mut self, canvas: &mut WindowCanvas);
    fn draw(&mut self, wndw: &mut WindowEvents, frame: Frame, canvas: &mut WindowCanvas);
    fn controls(&mut self, wndw: &mut WindowEvents, event: &Event);
}