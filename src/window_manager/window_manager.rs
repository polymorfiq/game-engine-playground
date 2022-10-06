use super::Frame;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::video::WindowBuilder;
use sdl2::render::WindowCanvas;

use crate::window_controller::WindowController;

pub struct WindowManager {
    ctx: Sdl,
    canvas: WindowCanvas,
    ctrl: Box<dyn WindowController>
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

impl WindowManager {
    pub fn new(ctx: Sdl, mut window_builder: WindowBuilder, ctrl: Box<dyn WindowController>) -> Self {
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
            ctrl: ctrl
        }
    }

    pub fn display_loop(&mut self) {
        let mut event_pump = self.ctx.event_pump().unwrap();
        let mut i = 0;

        self.ctrl.on_init(&mut self.canvas);

        loop {
            let frame = Frame{number: i};
            let mut events = WindowEvents{state: WindowState::Open};

            self.ctrl.controls(&mut events, &mut event_pump);
            self.ctrl.draw(&mut events, frame, &mut self.canvas);
            if let WindowState::Closed = events.state { break; }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            i = (i + 1) % 255;
        }
    }
}