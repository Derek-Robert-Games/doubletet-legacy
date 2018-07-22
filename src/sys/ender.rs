use specs::prelude::*;
use components as c;
use sys::drop::get_max_y_by_x;

pub struct Ender;

impl<'a> System<'a> for Ender {
    type SystemData = (ReadExpect<'a, c::KeysPressed>,
                       WriteExpect<'a, c::KillProgram>,
                       WriteStorage<'a, c::Position>,
                       WriteStorage<'a, c::Active>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut kill, positions, active) = data;
        if keys.escape {
            kill.0 = true;
        }

        let max_y_by_x = get_max_y_by_x(&active, &positions);

        for value in max_y_by_x.values() {
            if (*value as u32) == 0 {
                println!("U luuuuuuu-se");
                kill.0 = true
            }
        }

    }
}