mod window_manager;
mod window_controller;
use window_manager::WindowManager;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let game = window_controller::Game::new();
    let builder = video_subsystem.window("rust-sdl2 demo", 800, 600);
    let mut mgr = WindowManager::new(sdl_context, builder, Box::new(game));

    mgr.display_loop();

    Ok(())
}