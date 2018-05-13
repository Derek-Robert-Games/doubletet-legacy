extern crate piston_window;

use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::clear;
use piston_window::rectangle;

const WINDOW_HEIGHT: u32 = 480;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_DIMENSIONS: [u32; 2] = [WINDOW_WIDTH, WINDOW_HEIGHT];

fn main() { 
    let mut window: PistonWindow = {
        WindowSettings::new("First Window", WINDOW_DIMENSIONS)
            .exit_on_esc(true) // Hitting escape exits the game.
            .build()
            .unwrap()
    };

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            )
        });
    }

    println!("Hello World!") // test comment
}
