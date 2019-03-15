use components as c;
use resources as r;
use settings;
use specs::prelude::*;
use std::time::Instant;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, c::Active>,
        WriteStorage<'a, c::Position>,
        WriteExpect<'a, r::Clock>,
        WriteExpect<'a, r::Actions>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut active, mut pos, mut clock, mut actions) = data;

        let time_since_move = clock.last_player_move.elapsed();
        let secs_since_move =
            time_since_move.as_secs() as f64 + time_since_move.subsec_nanos() as f64 * 1e-9;

        for (active, pos) in (&mut active, &mut pos).join() {
            // Only move the active block
            if active.0 {
                let window_width: f64 = settings::WINDOW_WIDTH.into();
                if actions.move_right {
                    if secs_since_move > settings::MAX_MOVE_SPEED {
                        pos.x = pos.x + settings::RECT_WIDTH;
                        if pos.x > (window_width - settings::RECT_WIDTH) {
                            pos.x = 0.0;
                        }
                        clock.last_player_move = Instant::now();
                    }
                }
                if actions.move_left {
                    if secs_since_move > settings::MAX_MOVE_SPEED {
                        pos.x = pos.x - settings::RECT_WIDTH;
                        if pos.x < 0.0 {
                            pos.x = window_width - settings::RECT_WIDTH
                        }
                        clock.last_player_move = Instant::now();
                    }
                }
            }
        }
        actions.move_right = false;
        actions.move_left = false;
    }
}
