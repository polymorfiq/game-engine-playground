use sdl2::event::Event;
use super::Frame;

pub trait Window {
    fn process_event(&mut self, event: &Event);
    fn display(&mut self, frame: Frame);
    fn is_closed(&self) -> bool;
    fn is_focused(&self) -> bool;
}