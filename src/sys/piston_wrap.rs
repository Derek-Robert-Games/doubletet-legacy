use components as c;
use piston_window::*;
use resources as r;
use settings;
use specs::prelude::*;
use utils::Offset;
use Button;

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
        WriteExpect<'a, r::Actions>,
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

fn draw_shape(
    pos: &c::Position,
    dim: &c::Dimensions,
    color: &c::Color,
    offsets: &[Offset; 4],
    context: piston_window::context::Context,
    graphics: &mut piston_window::G2d,
) {
    for offset in offsets.iter() {
        let off_pos = pos.get_offset_position(offset);
        rectangle(
            build_color(&color),
            build_rect(&off_pos, &dim),
            context.transform,
            graphics,
        );
    }
}

fn build_rect(pos: &c::Position, dim: &c::Dimensions) -> [f64; 4] {
    [pos.x, pos.y, dim.width, dim.height]
}

fn build_color(color: &c::Color) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}
