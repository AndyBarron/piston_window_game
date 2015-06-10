extern crate gfx_device_gl;
extern crate gfx;
extern crate gfx_graphics;
extern crate piston_window;
pub use piston_window::*;

mod game_input;
mod screen;

pub use game_input::*;
pub use screen::*;

// use piston::window::WindowSettings;
// use piston::event::{Events, UpdateEvent, RenderEvent, UpdateArgs};
// use sdl2_window::Sdl2Window as PistonWindow;
// use opengl_graphics::{ GlGraphics, OpenGL };
// use screen::UpdateResult;

pub fn launch<S: Screen>(start: S, title: &str, w: u32, h: u32) {
    let mut cur_set = vec![WindowSettings::new(title, [w, h])];
    let mut screen: Box<Screen> = Box::new(start);
    'game: loop {
        let window: PistonWindow = cur_set.pop()
                .expect("ERROR: cur_set").into();
        let mut im = GameInput::new();
        'events: for e in window {
            e.event.as_ref().map(|evt| im.update(evt));
            let mut result = None;

            if let Some(args) = e.update_args() {
                result = Some(screen.update(&args, &im));
                im.end_frame();
            }
            if let Some(args) = e.render_args() {
                e.draw_2d(|c, gfx| {
                    screen.draw(&args, c, gfx);
                });
            }
            if result.is_none() { continue; }
            match result.unwrap() {
                UpdateResult::Done => {}
                UpdateResult::ChangeScreen(boxed) => {
                    screen = boxed;
                    screen.update(&UpdateArgs{dt: 0.}, &im);
                }
                UpdateResult::Quit => {
                    break 'game;
                }
                // TODO how to properly restart/resize window?
                // UpdateResult::ReloadWindow(new_set) => {
                //     cur_set.push(new_set);
                //     continue 'game;
                // }
            }
        }
        // if 'events ends, the window is closed
        break 'game;
    }

    screen.on_exit();
}