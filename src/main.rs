extern crate piston_window;

use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::clear;
use piston_window::rectangle;
use std::time::Instant;
use std::time::Duration;

const WINDOW_HEIGHT: u32 = 480;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_DIMENSIONS: [u32; 2] = [WINDOW_WIDTH, WINDOW_HEIGHT];

fn main() {
    demo();
}

fn demo() {
    let mut window: PistonWindow = {
        WindowSettings::new("First Window", WINDOW_DIMENSIONS)
            .exit_on_esc(true) // Hitting escape exits the game.
            .build()
            .unwrap()
    };

    // Initial time.
    let mut time_before =  Instant::now();

    // Initial state of the rect. (all values in pixels).
    // X and Y denote upper left hand side of the rectangle.
    // [x, y, width, height]
    let mut rect = [0.0, 0.0, 100.0, 100.0];
    let rect_color = [1.0, 0.0, 0.0, 1.0];

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            let elapsed = time_before.elapsed();
            time_before = Instant::now();

            // Clear all current drawings from the canvas.
            clear([1.0; 4], graphics);

            // Update the coordinates of the rectangle.
            update_rect(&mut rect, elapsed);
            rectangle(rect_color, rect, context.transform,graphics)
        });
    }

    println!("Running the demo!") // test comment
}

fn update_rect(rect: &mut [f64; 4], elapsed_time: Duration) {
    // pixels per second
    let speed = 200.0;
    let elapsed_nanos = elapsed_time.subsec_nanos();
    let nanoseconds_in_a_second = 1000000000.0;
    let y_delta = (elapsed_nanos as f64) * speed / nanoseconds_in_a_second;

    rect[1] = (rect[1] + y_delta) % (WINDOW_HEIGHT as f64);
}

