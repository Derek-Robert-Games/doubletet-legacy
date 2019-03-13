use resources::KillProgram;
use piston_window::PistonWindow;
use piston_window::WindowSettings;
use menu::components::MenuNavigation;
use menu::components::MenuItem;
use menu::sys::controller::MenuController;
use menu::sys::ender::MenuEnder;
use menu::sys::navigator::MenuNavigator;
use specs::prelude::*;
use settings;
use components::Active;
use menu::components::MenuCommands;

pub fn show_menu() {
    let window = init_menu_window();
    let mut world = init_menu_world();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MenuEnder, "menu_ender", &[])
        .with(MenuNavigator, "menu_navigator", &[])
        .with_thread_local(MenuController{ window: window })
        .build();

    while !world.read_resource::<KillProgram>().0 { //press esc while playing to end the loop
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}

fn init_menu_window() -> PistonWindow {
    let window: PistonWindow = {
        WindowSettings::new("DoubleTetMenu", settings::WINDOW_DIMENSIONS)
            .exit_on_esc(true)
            .build()
            .unwrap()
    };
    window
}

fn init_menu_world() -> World {
    let mut world = World::new();
    world.register::<MenuItem>();
    world.register::<Active>();
    world.add_resource(MenuNavigation {
        move_up: false,
        move_down: false,
        move_left: false,
        move_right: false,
        escape: false,
        enter: false
    });
    world.add_resource(MenuCommands {
        new_game: false,
        quit: false,
        print_lol: false
    });
    world.add_resource(KillProgram(false));

    world.create_entity().with(MenuItem {
        action: "new_game".to_string(),
        title: "New Game".to_string(),
        order: 0,
        id: 0
    }).with(Active(true)).build();
    world.create_entity().with(MenuItem {
        action: "do_nothing".to_string(),
        title: "This button does nothing".to_string(),
        order: 1,
        id: 1
    }).with(Active(false)).build();
    world.create_entity().with(MenuItem {
        action: "quit".to_string(),
        title: "Quit".to_string(),
        order: 2,
        id: 2
    }).with(Active(false)).build();

    world
}


