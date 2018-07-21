extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use piston_window::*;
use specs::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

/****** Constants ******/

// R: 800 by 640 seems to not quite fit on my laptop screen.
// Since window resize does not scale the map, blocks will be chopped off at the bottom of my screen.
const WINDOW_HEIGHT: u32 = 625;
const WINDOW_WIDTH: u32 = 500;
const WINDOW_DIMENSIONS: [u32; 2] = [WINDOW_WIDTH, WINDOW_HEIGHT];
const RECT_WIDTH: f64 = (WINDOW_WIDTH as f64) / 8.0;
const RECT_HEIGHT: f64 = (WINDOW_HEIGHT as f64) / 10.0;
const NANOS_PER_SECOND: f64 = 1000000000.0;
const MAX_MOVE_SPEED: f64 = 0.05;
const MAX_SPAWN_SPEED: f64 = 0.5;

/****** Components ******/

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
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Component, Debug)]
struct DropSpeed(f64);


#[derive(Component, Debug)]
struct Active(bool);

/****** Resources ******/
// These tend to be globals

struct Clock {
    start: Instant,
    last_player_move: Instant,
    last_drop: Instant,
    last_spawn: Instant,
}

struct KeysPressed {
    left: bool,
    right: bool,
    space: bool,
    escape: bool,
}

struct Actions {
    move_left: bool,
    move_right: bool,
    spawn_block: bool,
}

struct KillProgram(bool);

/****** Systems ******/

struct Dropper;

impl<'a> System<'a> for Dropper {
    type SystemData = (
        WriteStorage<'a, Active>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Clock>,
        ReadStorage<'a, DropSpeed>,
        WriteExpect<'a, Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut active, mut positions, mut clock, drop_speed, mut actions) = data;
        let time_since_drop = clock.last_drop.elapsed();

        // Build the max_y_by_x for each value.
        // Would like to separate this into another function or library for testability, but can't
        // figure out the function signature /shrug
        // In any case, we have a hashmap, bucketed by the x value, with the highest y value.
        let mut max_y_by_x = HashMap::<u32, f64>::new();
        for (active, pos) in (&active, &positions).join() {
            if !active.0 {
                let pos_x = &(pos.x as u32);
                match max_y_by_x.get(pos_x) {
                    Some(&pos_y) => {
                        if (pos.y - RECT_HEIGHT) < pos_y {
                            max_y_by_x.insert(*pos_x, pos.y - RECT_HEIGHT);
                        }
                    }
                    None => {
                        max_y_by_x.insert(*pos_x, pos.y - RECT_HEIGHT);
                    }
                }
            }
        }

        for (active, pos, drop_speed) in (&mut active, &mut positions, &drop_speed).join() {
            // Only drop the active block.
            if active.0 {
                // drop blocks down
                let y_delta = time_since_drop.subsec_nanos() as f64 * drop_speed.0 / NANOS_PER_SECOND;
                pos.y = (pos.y + y_delta) % (WINDOW_HEIGHT as f64);

                let y_max = match max_y_by_x.get(&(pos.x as u32)) {
                    Some(&pos_y) => pos_y,
                    None         => (WINDOW_HEIGHT as f64) - (RECT_HEIGHT)
                };

                if pos.y >= y_max {
                    // Block has hit bottom of screen.
                    pos.y = y_max; 
                    active.0 = false; 
                    actions.spawn_block = true;
                } 
                clock.last_drop = Instant::now();
            }
        }   
    }
}

struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Active>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Clock>,
        WriteExpect<'a, Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut active, mut pos, mut clock, mut actions) = data;

        let time_since_move = clock.last_player_move.elapsed();
        let secs_since_move =
            time_since_move.as_secs() as f64 + time_since_move.subsec_nanos() as f64 * 1e-9;

        for (active, pos) in (&mut active, &mut pos).join() {
            // Only move the active block
            if active.0 {
                let window_width: f64 = WINDOW_WIDTH.into();
                if actions.move_right {
                    if secs_since_move > MAX_MOVE_SPEED {
                        pos.x = pos.x + RECT_WIDTH;
                        if pos.x > (window_width - RECT_WIDTH) {
                            pos.x = 0.0;
                        }
                        clock.last_player_move = Instant::now();
                    }
                }
                if actions.move_left {
                    if secs_since_move > MAX_MOVE_SPEED {
                        pos.x = pos.x - RECT_HEIGHT;
                        if pos.x < 0.0 {
                            pos.x = window_width - RECT_WIDTH
                        }
                        clock.last_player_move = Instant::now();
                    }
                }
            }
        }
        actions.move_right = false;
        actions.move_left = false;
    }
}

struct BlockSpawner;

impl<'a> System<'a> for BlockSpawner {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Clock>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut clock, updater, mut actions) = data;

        let time_since_spawn = clock.last_spawn.elapsed();
        let secs_since_spawn =
            time_since_spawn.as_secs() as f64 + time_since_spawn.subsec_nanos() as f64 * 1e-9;

        if secs_since_spawn > MAX_SPAWN_SPEED {
            if actions.spawn_block {
                let new_block = entities.create();
                updater.insert(
                    new_block,
                    Dimensions {
                        width: RECT_WIDTH,
                        height: RECT_HEIGHT,
                    },
                );
                updater.insert(new_block, Position { x: 0.0, y: 0.0 });
                updater.insert(
                    new_block,
                    Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                );
                updater.insert(new_block, DropSpeed(100.0));
                updater.insert(new_block, Active(true));

                clock.last_spawn = Instant::now();
                actions.spawn_block = false;
            }
        }
    }
}

struct Printer;

impl<'a> System<'a> for Printer {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Dimensions>,
        ReadStorage<'a, Color>,
    );

    fn run(&mut self, (pos, dim, color): Self::SystemData) {
        for (pos, dim, color) in (&pos, &dim, &color).join() {
            println!("Printer -> {:?}", &dim);
            println!("Printer -> {:?}", &pos);
            println!("Printer -> {:?}", &color);
        }
    }
}

struct Timer;

impl<'a> System<'a> for Timer {
    type SystemData = (WriteExpect<'a, Clock>);

    fn run(&mut self, time: Self::SystemData) {
        // impl
    }
}

struct Render {
    window: PistonWindow,
}

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Dimensions>,
        ReadStorage<'a, Color>,
        WriteExpect<'a, KeysPressed>,
        WriteExpect<'a, Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, dim, color, mut keys, mut actions) = data;

        if let Some(event) = self.window.next() {
            // saving user Movement for process by other systems

            match event.press_args() {
                Some(Button::Keyboard(Key::Right)) => {
                    keys.right = true;
                    actions.move_right = true;
                }
                Some(Button::Keyboard(Key::Left)) => {
                    keys.left = true;
                    actions.move_left = true;
                }
                Some(Button::Keyboard(Key::Space)) => {
                    keys.space = true;
                    actions.spawn_block = true;
                }
                Some(Button::Keyboard(Key::Escape)) => {
                    keys.escape = true;
                }
                _ => {}
            }

            match event.release_args() {
                Some(Button::Keyboard(Key::Right)) => keys.right = false,
                Some(Button::Keyboard(Key::Left)) => keys.left = false,
                Some(Button::Keyboard(Key::Space)) => keys.space = false,
                Some(Button::Keyboard(Key::Escape)) => keys.escape = false,
                _ => {}
            }

            // updating graphics
            self.window.draw_2d(&event, |context, graphics| {
                clear([1.0; 4], graphics);

                //for all entities with pos, dim, and color properties (i.e. rect)
                for (pos, dim, color) in (&pos, &dim, &color).join() {
                    let temp_rect = [pos.x, pos.y, dim.width, dim.height];
                    let temp_color = [color.r, color.g, color.b, color.a];
                    rectangle(temp_color, temp_rect, context.transform, graphics);
                }
            });
        }
    }
}

struct Ender;

impl<'a> System<'a> for Ender {
    type SystemData = (ReadExpect<'a, KeysPressed>,
                        WriteExpect<'a, KillProgram>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut kill) = data;
        if keys.escape {
            kill.0 = true;
        }
    }
}


/****** Main ******/

fn main() {
    ecs_demo();
}

fn ecs_demo() {
    let window = init_window();
    let mut world = init_world();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Dropper, "dropper", &[])
        //.with(Printer, "Printer", &[]) // for debugging
        .with(Timer, "timer", &[])
        .with(BlockSpawner, "spawner", &[]) 
        .with(Movement, "movement", &[])
        .with(Ender, "ender", &[])
        .with_thread_local(Render{window})
        .build();

    while !world.read_resource::<KillProgram>().0 { //press esc while playing to end the loop
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}

fn init_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Dimensions>();
    world.register::<Color>();
    world.register::<DropSpeed>();
    world.register::<Active>();

    world.add_resource(KeysPressed {
        left: false,
        right: false,
        space: false,
        escape: false,
    });
    world.add_resource(Actions {
        move_left: false,
        move_right: false,
        spawn_block: false,
    });
    world.add_resource(Clock {
        start: Instant::now(),
        last_player_move: Instant::now(),
        last_drop: Instant::now(),
        last_spawn: Instant::now(),
    });
    world.add_resource(KillProgram(false));

    world
        .create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(Dimensions {
            width: RECT_WIDTH,
            height: RECT_HEIGHT,
        })
        .with(Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })
        .with(DropSpeed(100.0))
        .with(Active(true))
        .build();

    world
}

fn init_window() -> PistonWindow {
    let window: PistonWindow = {
        WindowSettings::new("DoubleTet", WINDOW_DIMENSIONS)
            .exit_on_esc(true)
            .build()
            .unwrap()
    };
    window
}
