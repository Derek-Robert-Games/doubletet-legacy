use specs::prelude::*;
use std::time::Instant;

/****** Components ******/

#[derive(Component, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Component, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Component, Debug)]
pub struct DropSpeed(pub f64);


#[derive(Component, Debug)]
pub struct Active(pub bool);

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
