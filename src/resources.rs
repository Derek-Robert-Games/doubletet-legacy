use settings;
use std::time::Instant;
use utils::*;

/****** Resources ******/

pub struct Clock {
    pub start: Instant,
    pub last_player_move: Instant,
    pub last_drop: Instant,
    pub last_spawn: Instant,
}

pub struct KeysPressed {
    pub left: bool,
    pub right: bool,
    pub space: bool,
    pub escape: bool,
}

pub struct Actions {
    pub move_left: bool,
    pub move_right: bool,
    pub spawn_block: bool,
}

pub struct KillProgram(pub bool);

pub struct GameMap {
    pub map: [[bool; (settings::NUMBER_OF_CELLS_HIGH as usize)];
        (settings::NUMBER_OF_CELLS_WIDE as usize)],
}

impl GameMap {
    pub fn get(&self, coords: &Coordinates) -> bool {
        if self.in_bounds(coords) {
            return self.map[(coords.x as usize)][(coords.y as usize)];
        }
        false
    }

    pub fn set(&mut self, coords: &Coordinates, value: bool) {
        if self.in_bounds(coords) {
            self.map[(coords.x as usize)][(coords.y as usize)] = value;
        }
    }

    pub fn limit_break(&self, coords: &Coordinates) -> bool {
        if coords.y < 0 {
            return true;
        }
        false
    }

    pub fn in_bounds(&self, coords: &Coordinates) -> bool {
        if coords.x < 0 || coords.y < 0 {
            return false;
        } else if coords.x >= (settings::NUMBER_OF_CELLS_WIDE as i16) {
            return false;
        } else if coords.y >= (settings::NUMBER_OF_CELLS_HIGH as i16) {
            return false;
        }
        true
    }
}
