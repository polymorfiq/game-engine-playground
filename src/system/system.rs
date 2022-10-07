use std::time::Duration;
use sdl2::EventPump;
use crate::window_manager::Frame;
use crate::window_manager::Window;

pub struct System {
    event_pump: Box<EventPump>,
    frame_num: u8,
    windows: Vec<Box<dyn Window>>
}

impl System {
    pub fn new(event_pump: Box<EventPump>) -> Self {
        Self{
            event_pump: event_pump,
            frame_num: 0,
            windows: vec![]
        }
    }

    pub fn push(&mut self, window: Box<dyn Window>) {
        self.windows.push(window)
    }

    pub async fn next(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            for i in 0..self.windows.len() {
                if self.windows[i].is_focused() {
                    self.windows[i].process_event(&event);
                }
            }
        }

        let mut stay_open = true;
        for i in 0..self.windows.len() {
            let frame = Frame{number: self.frame_num};

            if self.windows[i].is_focused() { self.windows[i].display(frame) };
            if self.windows[i].is_closed() { stay_open = false; }
        }

        self.frame_num = (self.frame_num + 1) % 255;

        async_std::task::sleep(Duration::new(0, 1_000_000_000u32 / 60)).await;
        stay_open
    }
}