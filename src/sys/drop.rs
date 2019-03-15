use components as c;
use resources as r;
use settings;
use specs::prelude::*;
use std::time::Instant;

pub struct Dropper;

impl<'a> System<'a> for Dropper {
    type SystemData = (
        WriteStorage<'a, c::Active>,
        WriteStorage<'a, c::Position>,
        ReadStorage<'a, c::BlockOffsets>,
        WriteExpect<'a, r::Clock>,
        ReadStorage<'a, c::DropSpeed>,
        WriteExpect<'a, r::Actions>,
        ReadExpect<'a, r::GameMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut active, mut positions, offsets, mut clock, drop_speed, mut actions, map) = data;
        let time_since_drop = clock.last_drop.elapsed();

        for (active, pos, drop_speed, offsets) in
            (&mut active, &mut positions, &drop_speed, &offsets).join()
        {
            // Only drop the active block.
            if active.0 {
                // drop blocks down
                let y_delta = time_since_drop.subsec_nanos() as f64 * drop_speed.0
                    / settings::NANOS_PER_SECOND;
                pos.y = (pos.y + y_delta) % (settings::WINDOW_HEIGHT as f64);

                for offset in offsets.0.iter() {
                    let x = pos.x + (offset.x as f64) * settings::RECT_WIDTH;
                    let y = pos.y + (offset.y as f64) * settings::RECT_HEIGHT;
                    let y_max = match map.0.get(&(x as u32)) {
                        Some(&pos_y) => pos_y,
                        None => (settings::WINDOW_HEIGHT as f64) - (settings::RECT_HEIGHT),
                    };
                    if y >= y_max {
                        pos.y = y_max;
                        active.0 = false;
                        actions.spawn_block = true;
                    }
                }

                clock.last_drop = Instant::now();
            }
        }
    }
}
