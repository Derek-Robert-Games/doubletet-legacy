use specs::prelude::*;
use menu::components::MenuNavigation;
use resources::KillProgram;

pub struct MenuEnder;

impl<'a> System<'a> for MenuEnder {
    type SystemData = (WriteExpect<'a, KillProgram>,
                       WriteExpect<'a, MenuNavigation>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut kill, menu_navigation) = data;
        if menu_navigation.escape {
            println!("killing");
            kill.0 = true;
        }
    }
}
