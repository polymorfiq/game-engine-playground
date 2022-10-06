use sdl2::{EventPump};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::window_manager::{Frame, WindowEvents};
use super::WindowController;

pub struct Game {
    pos: (usize, usize)
}

impl Game {
    pub const fn new() -> Self {
        Game{pos: (0, 0)}
    }
}

impl WindowController for Game {
    fn on_init(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
    }

    fn controls(&mut self, wndw: &mut WindowEvents, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    wndw.close()
                },

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.pos.1 += 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.pos.1 -= 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.pos.0 += 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.pos.0 -= 1;
                }

                _ => {}
            }
        }
    }

    fn draw(&mut self, _wndw: &mut WindowEvents, frame: Frame, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(frame.number, 64, 255 - frame.number));
        println!("Position: {:?}", self.pos);

        canvas.clear();
    }
}