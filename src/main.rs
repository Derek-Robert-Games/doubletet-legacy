extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use piston_window::*;
use std::time::Instant;
use std::time::Duration;
use specs::prelude::*;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_DIMENSIONS: [u32; 2] = [WINDOW_WIDTH, WINDOW_HEIGHT];
const RECT_WIDTH: f64 = 100.0;
const RECT_HEIGHT: f64 = 100.0;
const NANOS_PER_SECOND: f64 = 1000000000.0;

// components

#[derive(Component, Debug)] 
struct Position {
    x: f64,
    y: f64,
}

#[derive(Component, Debug)]
struct Dimensions {
    width: f64, 
    height: f64,
} 

#[derive(Component, Debug)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[derive(Component, Debug)]
struct Speed(f64);


// resources (components existing independent of entities)

struct Time {
    time_before: Instant, 
    time_elapsed: Duration,
}

struct KeysPressed {
    left: bool,
    right: bool,
}

struct Window(PistonWindow);


// systems

struct Slider;

impl<'a> System<'a> for Slider {
    type SystemData = (WriteStorage<'a, Position>,
                       ReadExpect<'a, Time>, //Read Expect, b/c Default not impl for Time (see chpt 6 of specs)
                       ReadStorage<'a, Speed>); 

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, time, speed) = data;
        let elapsed_nanos = time.time_elapsed.subsec_nanos();

        for (pos, speed) in (&mut pos, &speed).join() {
            let actual_speed = speed.0;
            let y_delta = (elapsed_nanos as f64) * actual_speed / NANOS_PER_SECOND;
            pos.y = (pos.y + y_delta) % (WINDOW_HEIGHT as f64);
        }
    }
}

struct Blitter;

impl<'a> System<'a> for Blitter {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Dimensions>,
                       ReadStorage<'a, Color>);

    fn run(&mut self, (pos, dim, color): Self::SystemData) {
        for (pos, dim, color) in (&pos, &dim, &color).join() {
            println!("Blitter -> {:?}", &dim);
            println!("Blitter -> {:?}", &pos);
            println!("Blitter -> {:?}", &color);
        }
    }
}

struct Timer;

impl<'a> System<'a> for Timer {
    type SystemData = (WriteExpect<'a, Time>); 

    fn run(&mut self, mut time: Self::SystemData) {
        time.time_elapsed = time.time_before.elapsed();
        time.time_before = std::time::Instant::now();
    }
}

struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Dimensions>,
                       ReadStorage<'a, Color>);

    fn run(&mut self, (pos, dim, color): Self::SystemData) {
        for (pos, dim, color) in (&pos, &dim, &color).join() {
        }
    }
}


//main

fn main() {
    ecs_demo();


/*  if we want to use parallel execution of systems, this can be done with specs::DispatcherBuilder

    let mut dispatcher = DispatcherBuilder::new()
        .with(Slider, "slider", &[])
        .with(Blitter, "blitter", &["slider"])
        .build();
    dispatcher.dispatch(&mut world.res);  */
    
    //demo();
}

fn ecs_demo() {
    let mut window = init_window();
    let mut world = init_world();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Slider, "slider", &[])
        .with(Blitter, "blitter", &[])
        .with(Timer, "timer", &[])
        .with_thread_local(Render)
        .build();

    println!("Running the demo!");
    while let Some(event) = window.next() {
        dispatcher.dispatch(&mut world.res); // runs all systems one cycle using multi-threading (except local threads)

       /*  // pattern matching on user input
        match event.press_args() {
            Some(Button::Keyboard(Key::Right)) => move_rect_right(&mut rect),
            Some(Button::Keyboard(Key::Left)) => move_rect_left(&mut rect),
            _ => {},
        } */
        
        //update the graphics window
        /* window.draw_2d(&event, |context, graphics| {
            // Clear all current drawings from the canvas.
            clear([1.0; 4], graphics);

            // Update the coordinates of the rectangle.
            rectangle(rect_color, rect, context.transform, graphics)
        });  */
    }
}

fn init_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Dimensions>();
    world.register::<Color>();
    world.register::<Speed>();

    let time_before = std::time::Instant::now();
    let time_elapsed = time_before.elapsed();
    world.add_resource( 
        Time { 
            time_before: time_before, 
            time_elapsed: time_elapsed }
    );

    let rect = world.create_entity()
        .with(Position{ x: 0.0, y: 0.0 })
        .with(Dimensions{ width: RECT_WIDTH, height: RECT_HEIGHT})
        .with(Color{ r: 1.0, g: 0.0, b: 0.0, a: 1.0 })
        .with(Speed(200.0))
        .build();
    
    world
}

fn init_window() -> PistonWindow {
    let mut window: PistonWindow = {
        WindowSettings::new("DoubleTet", WINDOW_DIMENSIONS)
            .exit_on_esc(true) // Hitting escape exits the game.
            .build()
            .unwrap()
    };
    window
}







//////////

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
    let mut rect = [0.0, 0.0, RECT_WIDTH, RECT_HEIGHT];
    let rect_color = [1.0, 0.0, 0.0, 1.0];

    println!("Running the demo!");
    while let Some(event) = window.next() {
        let elapsed_time = time_before.elapsed();
        time_before = Instant::now();
        slide_rect_down(&mut rect, elapsed_time);

        // pattern matching on user input
        match event.press_args() {
            Some(Button::Keyboard(Key::Right)) => move_rect_right(&mut rect),
            Some(Button::Keyboard(Key::Left)) => move_rect_left(&mut rect),
            _ => {},
        }
        
        //update the graphics window
        window.draw_2d(&event, |context, graphics| {
            // Clear all current drawings from the canvas.
            clear([1.0; 4], graphics);

            // Update the coordinates of the rectangle.
            rectangle(rect_color, rect, context.transform, graphics)
        });
    } 
}

fn slide_rect_down(rect: &mut [f64; 4], elapsed_time: Duration) {
    // pixels per second
    let speed = 200.0;
    let elapsed_nanos = elapsed_time.subsec_nanos();
    let nanoseconds_in_a_second = 1000000000.0;
    let y_delta = (elapsed_nanos as f64) * speed / nanoseconds_in_a_second;

    rect[1] = (rect[1] + y_delta) % (WINDOW_HEIGHT as f64);
}

///moves the rectangle to the right by RECT_WIDTH # of pixels
fn move_rect_right(rect: &mut [f64; 4]) {
    rect[0] = rect[0] + RECT_WIDTH;
    let window_width: f64 = WINDOW_WIDTH.into(); // converts to f64 for arithmetic
    if rect[0] > (window_width - RECT_WIDTH) {
        rect[0] = 0.0;
    }
}

///moves the rectangle to the left by RECT_WIDTH # of pixels
fn move_rect_left(rect: &mut[f64; 4]) {
    rect[0] = rect[0] - RECT_WIDTH;
    let window_width: f64 = WINDOW_WIDTH.into(); 
    if rect[0] < 0.0     {
        rect[0] = window_width - RECT_WIDTH;
    }
}
