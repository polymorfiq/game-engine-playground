use super::Frame;
use std::time::Duration;

use sdl2::{EventPump, Sdl};
use sdl2::video::WindowBuilder;
use sdl2::render::WindowCanvas;

pub struct WindowManager<'a> {
    ctx: Sdl,
    canvas: WindowCanvas,
    on_init: Box<dyn FnMut(&mut WindowCanvas) + 'a>,
    draw: Box<dyn FnMut(&mut WindowEvents, Frame, &mut WindowCanvas) + 'a>,
    controls: Box<dyn FnMut(&mut WindowEvents, &mut EventPump) + 'a>
}

pub struct WindowEvents {
    state: WindowState
}

enum WindowState {
    Open,
    Closed
}

impl WindowEvents {
    pub fn close(&mut self) {
        self.state = WindowState::Closed
    }
}

impl<'a> WindowManager<'a> {
    pub fn new(ctx: Sdl, mut window_builder: WindowBuilder) -> Self {
        let window = window_builder
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        WindowManager {
            ctx: ctx,
            canvas: canvas,
            on_init: Box::new(|_| {}),
            draw: Box::new(|_, _, _| {}),
            controls: Box::new(|_, _| {})
        }
    }

    pub fn on_init(&mut self, on_init: Box<dyn FnMut(&mut WindowCanvas) + 'a>) {
        self.on_init = on_init;
    }

    pub fn controls(&mut self, controls: Box<dyn FnMut(&mut WindowEvents, &mut EventPump) + 'a>) {
        self.controls = controls;
    }

    pub fn draw(&mut self, draw: Box<dyn FnMut(&mut WindowEvents, Frame, &mut WindowCanvas) + 'a>) {
        self.draw = draw;
    }

    pub fn display_loop(&mut self) {
        let mut event_pump = self.ctx.event_pump().unwrap();
        let mut i = 0;

        (*self.on_init)(&mut self.canvas);

        loop {
            let frame = Frame{number: i};
            let mut events = WindowEvents{state: WindowState::Open};

            (*self.controls)(&mut events, &mut event_pump);
            (*self.draw)(&mut events, frame, &mut self.canvas);
            if let WindowState::Closed = events.state { break; }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            i = (i + 1) % 255;
        }
    }
}