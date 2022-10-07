use sdl2::event::Event;
use sdl2::video::WindowBuilder;
use sdl2::render::WindowCanvas;

use super::Frame;
use super::Window;
use crate::window_controller::WindowController;

pub struct WindowManager<Ctrl: WindowController> {
    canvas: WindowCanvas,
    ctrl: Ctrl,
    state: WindowState
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

impl<Ctrl: WindowController> WindowManager<Ctrl> {
    pub fn new(window_builder: &mut WindowBuilder, mut ctrl: Ctrl) -> Self {
        let window = window_builder
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        ctrl.on_init(&mut canvas);

        WindowManager {
            canvas: canvas,
            ctrl: ctrl,
            state: WindowState::Open
        }
    }
}

impl<Ctrl: WindowController> Window for WindowManager<Ctrl> {
    fn process_event(&mut self, event: &Event) {
        let mut wndw = WindowEvents{state: WindowState::Open};
        self.ctrl.controls(&mut wndw, event);
        if matches!(wndw.state, WindowState::Closed) { self.state = wndw.state; }
    }

    fn display(&mut self, frame: Frame) {
        let mut wndw = WindowEvents{state: WindowState::Open};

        self.ctrl.draw(&mut wndw, frame, &mut self.canvas);
        if matches!(wndw.state, WindowState::Closed) { self.state = wndw.state; }

        self.canvas.present();
    }

    fn is_closed(&self) -> bool {
        return matches!(self.state, WindowState::Closed)
    }

    fn is_focused(&self) -> bool {
        sdl2::sys::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32 & self.canvas.window().window_flags() > 0
    }
}