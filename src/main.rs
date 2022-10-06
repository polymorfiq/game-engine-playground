use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod window_manager;
use window_manager::WindowManager;
use std::{cell::RefCell, rc::Rc};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window_builder = video_subsystem.window("rust-sdl2 demo", 800, 600);
    let pos = Rc::new(RefCell::new((0, 0)));

    let mut mgr = WindowManager::new(sdl_context, window_builder);
    mgr.on_init(Box::new(|canvas| {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
    }));

    mgr.controls(Box::new(|window_evts, event_pump| {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    window_evts.close()
                },

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    pos.borrow_mut().1 += 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    pos.borrow_mut().1 -= 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    pos.borrow_mut().0 += 1;
                }

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    pos.borrow_mut().0 -= 1;
                }

                _ => {}
            }
        }
    }));

    mgr.draw(Box::new(|_window_evts, frame, canvas| {
        canvas.set_draw_color(Color::RGB(frame.number, 64, 255 - frame.number));
        println!("Position: {:?}", pos.borrow());

        canvas.clear();
    }));

    mgr.display_loop();

    Ok(())
}