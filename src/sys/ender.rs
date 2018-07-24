use specs::prelude::*;
use resources as r;

pub struct Ender;

impl<'a> System<'a> for Ender {
    type SystemData = (ReadExpect<'a, r::KeysPressed>,
                        WriteExpect<'a, r::KillProgram>,
                        ReadExpect<'a, r::GameMap>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut kill, map) = data;
        if keys.escape {
            kill.0 = true;
        }

        for value in map.0.values() {
            if (*value as u32) == 0 {
                println!("U luuuuuuu-se");
                kill.0 = true
            }
        }

    }
}