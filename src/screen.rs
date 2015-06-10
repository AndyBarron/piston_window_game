use piston_window::*;
use gfx_graphics::*;
use gfx_device_gl::{Resources, CommandBuffer, Output};
use super::game_input::GameInput;

pub enum UpdateResult {
    Done,
    Quit,
    ChangeScreen(Box<Screen>),
    // ReloadWindow(WindowSettings), // TODO
}

pub type GameGraphics<'a> = GfxGraphics<'a, Resources, CommandBuffer, Output>;

pub trait Screen {
    /* Required */
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
            -> UpdateResult;
    fn draw(&mut self, args: &RenderArgs, c: Context, gfx: &mut GameGraphics);
    /* Optional */
    fn on_exit(&self) {}
}