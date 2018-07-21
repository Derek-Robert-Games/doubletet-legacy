use specs::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
use components as c;
use settings;

pub struct Dropper; 

impl<'a> System<'a> for Dropper {
    type SystemData = (
        WriteStorage<'a, c::Active>,
        WriteStorage<'a, c::Position>,
        WriteExpect<'a, c::Clock>,
        ReadStorage<'a, c::DropSpeed>,
        WriteExpect<'a, c::Actions>,
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
            max_y_by_x = map_check(active.0, pos.x as u32, pos.y, max_y_by_x);
        }

        for (active, pos, drop_speed) in (&mut active, &mut positions, &drop_speed).join() {
            // Only drop the active block.
            if active.0 {
                // drop blocks down
                let y_delta = time_since_drop.subsec_nanos() as f64 * drop_speed.0 / settings::NANOS_PER_SECOND;
                pos.y = (pos.y + y_delta) % (settings::WINDOW_HEIGHT as f64);

                let y_max = match max_y_by_x.get(&(pos.x as u32)) {
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

fn map_check(active: bool, x: u32, y: f64, mut map: HashMap<u32, f64>) -> HashMap<u32, f64> {
    if !active {
        match map.get(&x) {
            Some(&y_found) => {
                if (y - settings::RECT_HEIGHT) < y_found {
                    map.insert(x, y - settings::RECT_HEIGHT);
                }
            }
            None => {
                map.insert(x, y - settings::RECT_HEIGHT);
            }
        }
    }
    map
}


