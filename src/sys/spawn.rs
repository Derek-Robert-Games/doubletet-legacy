use specs::prelude::*;
use settings;
use components as c;
use resources as r;
use std::time::Instant;

pub struct BlockSpawner;

impl<'a> System<'a> for BlockSpawner {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, r::Clock>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, r::Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut clock, updater, mut actions) = data;

        let time_since_spawn = clock.last_spawn.elapsed();
        let secs_since_spawn =
            time_since_spawn.as_secs() as f64 + time_since_spawn.subsec_nanos() as f64 * 1e-9;

        if secs_since_spawn > settings::MAX_SPAWN_SPEED {
            if actions.spawn_block {
                let new_block = entities.create();
                updater.insert(
                    new_block,
                    c::Dimensions {
                        width: settings::RECT_WIDTH,
                        height: settings::RECT_HEIGHT,
                    },
                );
                updater.insert(new_block, c::Position { x: 0.0, y: 0.0 });
                updater.insert(
                    new_block,
                    c::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                );
                updater.insert(new_block, c::DropSpeed(settings::STANDARD_DROP_SPEED));
                updater.insert(new_block, c::Active(true));

                clock.last_spawn = Instant::now();
                actions.spawn_block = false;
            }
        }
    }
}