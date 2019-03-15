use components as c;
use resources as r;
use settings;
use specs::prelude::*;
use std::collections::HashMap;

pub struct Mapper;

impl<'a> System<'a> for Mapper {
    type SystemData = (
        WriteStorage<'a, c::Position>,
        ReadStorage<'a, c::BlockOffsets>,
        WriteExpect<'a, r::GameMap>,
        ReadStorage<'a, c::Active>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, offsets, mut map, actives) = data;

        // for all active blocks, update the map with their y value
        for (active, pos, offsets) in (&actives, &positions, &offsets).join() {
            update_map(active.0, pos.x as u32, pos.y, &mut map.0, &offsets.0);
        }
    }
}

fn update_map(active: bool, x: u32, y: f64, map: &mut HashMap<u32, f64>, offsets: &[c::Offset; 4]) {
    if !active {
        for offset in offsets {
            let x_offset = (offset.x as f64) * settings::RECT_WIDTH;
            let y_offset = (offset.y as f64) * settings::RECT_HEIGHT;
            add_to_map(x + (x_offset as u32), y + y_offset, map)
        }
    }
}

fn add_to_map(x: u32, y: f64, map: &mut HashMap<u32, f64>) {
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
