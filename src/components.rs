use specs::prelude::*;

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