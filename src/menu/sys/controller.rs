extern crate find_folder;

use specs::prelude::*;
use menu::components::MenuNavigation;
use menu::components::MenuItem;
use piston_window::*;
use settings::WINDOW_WIDTH;
use std::path::PathBuf;
use components::Active;
use menu::components::MenuCommands;
use resources::KillProgram;
use ecs_demo;

pub struct MenuController {
    pub window: PistonWindow,
}

const MENU_ITEM_TEXT_COLOR_INACTIVE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const MENU_ITEM_TEXT_COLOR_ACTIVE: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

struct MenuKeysPressed {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    escape: bool,
    enter: bool
}

impl<'a> System<'a> for MenuController {
    type SystemData = (WriteExpect<'a, MenuNavigation>,
                       WriteExpect<'a, MenuCommands>,
                       ReadStorage<'a, MenuItem>,
                       ReadStorage<'a, Active>,
                       WriteExpect<'a, KillProgram>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut actions,
            mut menu_commands,
            menu_items,
            active,
            mut kill_program) = data;

        let mut keys =  MenuKeysPressed{
            left: false,
            right: false,
            up: false,
            down: false,
            escape: false,
            enter: false
        };

        if let Some(event) = self.window.next() {
            match event.press_args() {
                Some(Button::Keyboard(Key::Right)) => {
                    keys.right = true;
                    actions.move_right = true;
                }
                Some(Button::Keyboard(Key::Left)) => {
                    keys.left = true;
                    actions.move_left = true;
                }
                Some(Button::Keyboard(Key::Up)) => {
                    keys.up = true;
                    actions.move_up = true;
                }
                Some(Button::Keyboard(Key::Down)) => {
                    keys.down = true;
                    actions.move_down = true;
                }
                Some(Button::Keyboard(Key::Escape)) => {
                    keys.escape = true;
                    actions.escape = true;
                }
                Some(Button::Keyboard(Key::Return)) => {
                    keys.enter = true;
                    actions.enter = true;
                }
                _ => {}
            }

            match event.release_args() {
                Some(Button::Keyboard(Key::Right)) => keys.right = false,
                Some(Button::Keyboard(Key::Left)) => keys.left = false,
                Some(Button::Keyboard(Key::Up)) => keys.up = false,
                Some(Button::Keyboard(Key::Down)) => keys.down = false,
                Some(Button::Keyboard(Key::Escape)) => keys.escape = false,
                Some(Button::Keyboard(Key::Return)) => keys.enter = false,
                _ => {}
            }

            let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets").unwrap();
            let ref font = assets.join("28 Days Later.ttf");
            let factory = self.window.factory.clone();
            let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

            self.window.draw_2d(&event, |context, graphics| {
                clear([1.0; 4], graphics);
                for (menu_item, active) in (&menu_items, &active).join() {
                    let margin_pixels = 10;
                    let width = WINDOW_WIDTH - (2 * margin_pixels);
                    let height = 50;

                    let x_start = margin_pixels;
                    let y_start = ((menu_item.order as u32) * margin_pixels) + margin_pixels + height * (menu_item.order as u32);

                    let coordinates = [
                        x_start as f64,
                        y_start as f64,
                        width as f64,
                        height as f64
                    ];

                    rectangle([1.0, 0.0, 0.0, 1.0], coordinates, context.transform, graphics);
                    let text_transform = context.transform.trans(x_start as f64, y_start as f64 + 32.0);

                    let color = if active.0 {
                        MENU_ITEM_TEXT_COLOR_ACTIVE
                    } else {
                        MENU_ITEM_TEXT_COLOR_INACTIVE
                    };

                    let result = text::Text::new_color(color, 16).draw(
                        menu_item.title.as_str(),
                        &mut glyphs,
                        &context.draw_state,
                        text_transform,
                        graphics
                    );
                    result.ok();
                }

//                //for all entities with pos, dim, and color properties (i.e. rect)
//                for (pos, dim, color) in (&pos, &dim, &color).join() {
//                    let temp_rect = [pos.x, pos.y, dim.width, dim.height];
//                    let temp_color = [color.r, color.g, color.b, color.a];
//                    rectangle(temp_color, temp_rect, context.transform, graphics);
//                }
            });

            if menu_commands.has_command() {
                if menu_commands.new_game {
                    menu_commands.clear_all_commands();
                    ecs_demo();
                } else if menu_commands.quit {
                    kill_program.0 = true;
                } else if menu_commands.print_lol {
                    menu_commands.clear_all_commands();
                    println!("lol!");
                }
            }
        }
    }
}
