use components as c;
use resources as r;
use specs::prelude::*;
use utils::Offset;

pub struct Mapper;

impl<'a> System<'a> for Mapper {
    type SystemData = (
        WriteStorage<'a, c::Position>,
        ReadStorage<'a, c::BlockOffsets>,
        WriteExpect<'a, r::GameMap>,
        ReadStorage<'a, c::Active>,
        WriteExpect<'a, r::KillProgram>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, offsets, mut map, actives, mut kill) = data;

        // for all inactive blocks, update the map with their values
        for (active, pos, offsets) in (&actives, &positions, &offsets).join() {
            update_map(active.0, pos, &mut map, &offsets.0, &mut kill);
        }
    }
}

fn update_map(
    active: bool,
    pos: &c::Position,
    map: &mut WriteExpect<r::GameMap>,
    offsets: &[Offset; 4],
    kill: &mut WriteExpect<r::KillProgram>,
) {
    if !active {
        for offset in offsets {
            let coords = pos.get_offset_coords(offset);
            if map.limit_break(&coords) {
                kill.0 = true;
            }
            map.set(&coords, true);
        }
    }
}
