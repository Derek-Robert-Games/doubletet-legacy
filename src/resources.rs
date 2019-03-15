use settings;
use std::collections::HashMap;
use std::time::Instant;

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

pub struct GameMap(pub HashMap<u32, f64>);

pub struct GameMap2(
    pub  [[i8; (settings::NUMBER_OF_CELLS_HIGH as usize)];
        (settings::NUMBER_OF_CELLS_WIDE as usize)],
);
