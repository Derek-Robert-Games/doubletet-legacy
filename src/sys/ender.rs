use specs::prelude::*;
use components as c;

pub struct Ender;

impl<'a> System<'a> for Ender {
    type SystemData = (ReadExpect<'a, c::KeysPressed>,
                        WriteExpect<'a, c::KillProgram>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut kill) = data;
        if keys.escape {
            kill.0 = true;
        }
    }
}