use resources as r;
use specs::prelude::*;

pub struct Ender;

impl<'a> System<'a> for Ender {
    type SystemData = (
        ReadExpect<'a, r::KeysPressed>,
        WriteExpect<'a, r::KillProgram>,
        ReadExpect<'a, r::GameMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut kill, map) = data;
        if keys.escape {
            kill.0 = true;
        }
    }
}
