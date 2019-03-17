use components as c;
use resources as r;
use settings;
use specs::prelude::*;
use std::time::Instant;
use utils::Offset;

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
                let new_block = spawn(&updater, &entities);
                make_l_block(&updater, &new_block);
                clock.last_spawn = Instant::now();
                actions.spawn_block = false;
            }
        }
    }
}

fn spawn(updater: &Read<LazyUpdate>, entities: &Entities) -> specs::Entity {
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
    new_block
}

fn make_l_block(updater: &Read<LazyUpdate>, entity: &Entity) {
    updater.insert(
        *entity,
        c::BlockOffsets([
            Offset { x: 0, y: 0 },
            Offset { x: 1, y: 0 },
            Offset { x: 0, y: -1 },
            Offset { x: 0, y: -2 },
        ]),
    );
}
