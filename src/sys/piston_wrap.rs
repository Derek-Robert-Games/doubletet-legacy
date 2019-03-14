use specs::prelude::*;
use piston_window::*;
use Button;
use components as c;
use resources as r;
use settings;

pub struct PistonWrapper {
    pub window: PistonWindow,
}

impl<'a> System<'a> for PistonWrapper {
    type SystemData = (
        ReadStorage<'a, c::Position>,
        ReadStorage<'a, c::Dimensions>,
        ReadStorage<'a, c::BlockOffsets>,
        ReadStorage<'a, c::Color>,
        WriteExpect<'a, r::KeysPressed>,
        WriteExpect<'a, r::Actions>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, dim, offsets, color, mut keys, mut actions) = data;

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

            self.window.draw_2d(&event, |context, graphics| {
                clear_window(graphics);
                for (pos, dim, color, offsets) in (&pos, &dim, &color, &offsets).join() {
                    draw_shape(pos, dim, color, &offsets.0, context, graphics);
                } 
            });
        }
    }
}

fn clear_window(graphics: &mut piston_window::G2d) {
    clear([1.0; 4], graphics);
}

fn draw_shape(pos: &c::Position, dim: &c::Dimensions, color: &c::Color, offsets: &[c::Offset; 4], 
                context: piston_window::context::Context, graphics: &mut piston_window::G2d) {
    for offset in offsets.iter() {
        let x = pos.x + (offset.x as f64) * settings::RECT_WIDTH;
        let y = pos.y + (offset.y as f64) * settings::RECT_HEIGHT;
        let temp_rect = [x, y, dim.width, dim.height];
        let temp_color = [color.r, color.g, color.b, color.a];
        rectangle(temp_color, temp_rect, context.transform, graphics);
    }
}