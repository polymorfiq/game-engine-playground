use futures::executor::block_on;

mod window_manager;
mod window_controller;
mod system;
use window_manager::WindowManager;
use window_controller::Game;
use system::System;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    let mut builder1 = video_subsystem
        .window("rust-sdl2 window 1", 800, 600);

    let mut builder2 = video_subsystem
        .window("rust-sdl2 window 2", 800, 600);

    let mgr1: WindowManager<Game> = {
        let game = Game::new();
        WindowManager::new(
            builder1.position(200, 200),
            game
        )
    };

    let mgr2: WindowManager<Game> = {
        let game = Game::new();
        WindowManager::new(
            builder2.position(50, 50),
            game
        )
    };

    let mut system = System::new(Box::new(event_pump));
    system.push(Box::new(mgr1));
    system.push(Box::new(mgr2));

    loop {
        if !block_on(system.next()) { break; }
    }
    
    Ok(())
}