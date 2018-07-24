use specs::prelude::*;
use piston_window::*;
use Button;
use components as c;
use resources as r;

pub struct PistonWrapper {
    pub window: PistonWindow,
}

impl<'a> System<'a> for PistonWrapper {
    type SystemData = (
        ReadStorage<'a, c::Position>,
        ReadStorage<'a, c::Dimensions>,
        ReadStorage<'a, c::Color>,
        WriteExpect<'a, r::KeysPressed>,
        WriteExpect<'a, r::Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, dim, color, mut keys, mut actions) = data;

        if let Some(event) = self.window.next() {
            // saving user Movement for process by other systems

            match event.press_args() {
                Some(Button::Keyboard(Key::Right)) => {
                    keys.right = true;
                    actions.move_right = true;
                }
                Some(Button::Keyboard(Key::Left)) => {
                    keys.left = true;
                    actions.move_left = true;
                }
                Some(Button::Keyboard(Key::Space)) => {
                    keys.space = true;
                    actions.spawn_block = true;
                }
                Some(Button::Keyboard(Key::Escape)) => {
                    keys.escape = true;
                }
                _ => {}
            }

            match event.release_args() {
                Some(Button::Keyboard(Key::Right)) => keys.right = false,
                Some(Button::Keyboard(Key::Left)) => keys.left = false,
                Some(Button::Keyboard(Key::Space)) => keys.space = false,
                Some(Button::Keyboard(Key::Escape)) => keys.escape = false,
                _ => {}
            }

            // updating graphics
            self.window.draw_2d(&event, |context, graphics| {
                clear([1.0; 4], graphics);

                //for all entities with pos, dim, and color properties (i.e. rect)
                for (pos, dim, color) in (&pos, &dim, &color).join() {
                    let temp_rect = [pos.x, pos.y, dim.width, dim.height];
                    let temp_color = [color.r, color.g, color.b, color.a];
                    rectangle(temp_color, temp_rect, context.transform, graphics);
                }
            });
        }
    }
}