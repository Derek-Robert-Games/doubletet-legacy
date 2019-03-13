use specs::prelude::*;
use menu::components::MenuCommands;
use menu::components::MenuItem;
use menu::components::MenuNavigation;
use components::Active;

pub struct MenuNavigator;

impl<'a> System<'a> for MenuNavigator {
    type SystemData = (WriteExpect<'a, MenuNavigation>,
                       WriteExpect<'a, MenuCommands>,
                       WriteStorage<'a, MenuItem>,
                       WriteStorage<'a, Active>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut menu_navigation,
            mut menu_commands,
            menu_items,
            mut active) = data;

        if menu_navigation.has_movement_action() {
            let mut current_menu_id = 0;
            let mut max_id: i8 = -1;

            // Get current, max id.
            for (menu_item, active) in (&menu_items, &mut active).join() {
                max_id += 1;
                if active.0 {
                    current_menu_id = menu_item.id as i8;
                }
            }

            // Determine the new menu id.
            let new_menu_id: i8 = if menu_navigation.move_up {
                current_menu_id as i8 - 1
            } else if menu_navigation.move_down {
                current_menu_id as i8 + 1
            } else {
                current_menu_id as i8
            };

            // Fix overflow issues with the id.
            let new_menu_id_final = if new_menu_id < 0 {
                max_id as u8
            } else if new_menu_id > max_id {
                0
            } else {
                new_menu_id as u8
            };

            for (menu_item, active) in (&menu_items, &mut active).join() {
                active.0 = menu_item.id == new_menu_id_final;
            }

            menu_navigation.clear_all_actions();
        } else if menu_navigation.enter {
            menu_navigation.clear_all_actions();

            for (menu_item, active) in (&menu_items, &mut active).join() {
                if active.0 {
                    if menu_item.action == "quit" {
                        menu_commands.quit = true;
                    } else if menu_item.action == "new_game" {
                        menu_commands.new_game = true;
                    } else if menu_item.action == "do_nothing" {
                        menu_commands.print_lol = true;
                    }
                }
            }
        }
    }
}
