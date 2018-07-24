use specs::prelude::*;
use std::collections::HashMap;
use components as c;
use resources as r;
use settings;

pub struct Mapper; 

impl<'a> System<'a> for Mapper {
    type SystemData = (
        WriteStorage<'a, c::Position>,
        WriteExpect<'a, r::GameMap>,
        ReadStorage<'a, c::Active>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut map, actives) = data;

        // for all active blocks, update the map with their y value
        for (active, pos) in (&actives, &positions).join() {
            update_map(active.0, pos.x as u32, pos.y, &mut map.0);
        }   
    }
}

fn update_map(active: bool, x: u32, y: f64, map: &mut HashMap<u32, f64>) {
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
}
