use specs::prelude::*;
use std::time::Instant;
use components as c;
use resources as r;
use settings;

pub struct Dropper; 

impl<'a> System<'a> for Dropper {
    type SystemData = (
        WriteStorage<'a, c::Active>,
        WriteStorage<'a, c::Position>,
        WriteExpect<'a, r::Clock>,
        ReadStorage<'a, c::DropSpeed>,
        WriteExpect<'a, r::Actions>,
        ReadExpect<'a, r::GameMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut active, mut positions, mut clock, drop_speed, mut actions, map) = data;
        let time_since_drop = clock.last_drop.elapsed();

        for (active, pos, drop_speed) in (&mut active, &mut positions, &drop_speed).join() {
            // Only drop the active block.
            if active.0 {
                // drop blocks down
                let y_delta = time_since_drop.subsec_nanos() as f64 * drop_speed.0 / settings::NANOS_PER_SECOND;
                pos.y = (pos.y + y_delta) % (settings::WINDOW_HEIGHT as f64);

                // compare active block with existing blocks on map 
                let y_max = match map.0.get(&(pos.x as u32)) {
                    Some(&pos_y) => pos_y,
                    None         => (settings::WINDOW_HEIGHT as f64) - (settings::RECT_HEIGHT)
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

