extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod components;
mod resources;
mod sys;

use components as c;
use piston_window::*;
use resources as r;
use specs::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

/****** Constants ******/

// R: 800 by 640 seems to not quite fit on my laptop screen.
// Since window resize does not scale the map, blocks will be chopped off at the bottom of my screen.
mod settings {
    pub const WINDOW_HEIGHT: u32 = 600;
    pub const WINDOW_WIDTH: u32 = 300;
    pub const WINDOW_DIMENSIONS: [u32; 2] = [WINDOW_WIDTH, WINDOW_HEIGHT];
    pub const NUMBER_OF_CELLS_WIDE: u16 = 10;
    pub const NUMBER_OF_CELLS_HIGH: u16 = 20;
    pub const RECT_WIDTH: f64 = (WINDOW_WIDTH as f64) / (NUMBER_OF_CELLS_WIDE as f64);
    pub const RECT_HEIGHT: f64 = (WINDOW_HEIGHT as f64) / (NUMBER_OF_CELLS_HIGH as f64);
    pub const NANOS_PER_SECOND: f64 = 1000000000.0;
    pub const MAX_MOVE_SPEED: f64 = 0.05;
    pub const MAX_SPAWN_SPEED: f64 = 0.5;
    pub const STANDARD_DROP_SPEED: f64 = 200.0;
}

/****** Main ******/

fn main() {
    ecs_demo();
}

fn ecs_demo() {
    let window = init_window();
    let mut world = init_world();

    let mut dispatcher = DispatcherBuilder::new()
        .with(sys::drop::Dropper, "dropper", &[])
        .with(sys::spawn::BlockSpawner, "spawner", &[])
        .with(sys::movement::Movement, "movement", &[])
        .with(sys::ender::Ender, "ender", &[])
        .with(sys::map::Mapper, "mapper", &[])
        .with_thread_local(sys::piston_wrap::PistonWrapper { window: window })
        .build();

    while !world.read_resource::<r::KillProgram>().0 {
        //press esc while playing to end the loop
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}

fn init_world() -> World {
    let mut world = World::new();
    world.register::<c::Position>();
    world.register::<c::Dimensions>();
    world.register::<c::Color>();
    world.register::<c::DropSpeed>();
    world.register::<c::Active>();
    world.register::<c::BlockOffsets>();

    world.add_resource(r::KeysPressed {
        left: false,
        right: false,
        space: false,
        escape: false,
    });
    world.add_resource(r::Actions {
        move_left: false,
        move_right: false,
        spawn_block: false,
    });
    world.add_resource(r::Clock {
        start: Instant::now(),
        last_player_move: Instant::now(),
        last_drop: Instant::now(),
        last_spawn: Instant::now(),
    });
    world.add_resource(r::KillProgram(false));
    world.add_resource(r::GameMap(HashMap::<u32, f64>::new()));

    world
        .create_entity()
        .with(c::Position { x: 0.0, y: 0.0 })
        .with(c::Dimensions {
            width: settings::RECT_WIDTH,
            height: settings::RECT_HEIGHT,
        })
        .with(c::BlockOffsets([
            c::Offset { x: 0, y: 0 },
            c::Offset { x: 1, y: 0 },
            c::Offset { x: 0, y: -1 },
            c::Offset { x: 0, y: -2 },
        ]))
        .with(c::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })
        .with(c::DropSpeed(settings::STANDARD_DROP_SPEED))
        .with(c::Active(true))
        .build();

    world
}

fn init_window() -> PistonWindow {
    let window: PistonWindow = {
        WindowSettings::new("DoubleTet", settings::WINDOW_DIMENSIONS)
            .exit_on_esc(true)
            .build()
            .unwrap()
    };
    window
}
