extern crate piston_window_game;
use piston_window_game::*;
use piston_window_game::types::{Vec2d, Color};


#[test]
fn launch_test_screen() {
    launch(TestScreen::default(), "Test Screen", 400, 300)
}

const QUIT_AFTER: f64 = 3.0;

const COLORS: [Color; 3] = [
    [0.5, 0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0, 1.0],
    [0.0, 0.0, 0.5, 1.0],
];

pub struct TestScreen {
    color_idx: usize,
    position: Vec2d,
    rotation: f64, // radians
    age: f64, // seconds
}

impl Default for TestScreen {
    fn default() -> Self {
        Self::new(0, 0.)
    }
}

impl TestScreen {
    fn new(idx: usize, rot: f64) -> Self {
        TestScreen {
            color_idx: idx,
            position: [0., 0.],
            rotation: rot,
            age: 0.,
        }
    }
}

impl Screen for TestScreen {
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
        -> UpdateResult
    {
        if self.age > QUIT_AFTER || im.was_key_pressed(&Key::Escape) { 
            UpdateResult::Quit
        } else if im.was_key_pressed(&Key::R) {
            // TODO
            // UpdateResult::ReloadWindow(WindowSettings::new(
            //     "Resized",
            //     [300,300]
            // ))
            UpdateResult::Done
        } else if im.was_mouse_pressed(&MouseButton::Left) {
            let i = (self.color_idx + 1) % COLORS.len();
            UpdateResult::ChangeScreen(
                    Box::new(TestScreen::new(i, self.rotation)))
        } else {
            self.age += args.dt;
            self.position = im.mouse_position();
            self.rotation += 2.0 * args.dt;
            UpdateResult::Done
        }
    }
    fn draw(&mut self, _: &RenderArgs, c: Context, gl: &mut GameGraphics) {
        clear(COLORS[self.color_idx], gl);
        let square = rectangle::square(0.0, 0.0, 50.0);
        let col = COLORS[(self.color_idx + 1) % COLORS.len()];
        let transform = c.transform.trans(self.position[0], self.position[1])
                .rot_rad(self.rotation)
                .trans(-25.0, -25.0);
        rectangle(col, square, transform, gl);
    }
}