use settings;
use specs::prelude::*;
use utils::*;

/****** Components ******/

#[derive(Component, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn get_coords(&self) -> Coordinates {
        let mut x = 0.0;
        let mut y = 0.0;
        if !(self.x == 0.0) {
            x = self.x / (settings::RECT_WIDTH as f64);
        }
        if !(self.y == 0.0) {
            y = self.y / (settings::RECT_HEIGHT as f64);
        }
        Coordinates {
            x: (x as i16),
            y: (y as i16),
        }
    }

    pub fn get_offset_coords(&self, offset: &Offset) -> Coordinates {
        let coords = self.get_coords();
        Coordinates {
            x: coords.x + (offset.x as i16),
            y: coords.y + (offset.y as i16),
        }
    }

    pub fn get_offset_position(&self, offset: &Offset) -> Position {
        Position {
            x: self.x + (offset.x as f64) * settings::RECT_WIDTH,
            y: self.y + (offset.y as f64) * settings::RECT_HEIGHT,
        }
    }
}

#[derive(Component, Debug)]
pub struct BlockOffsets(pub [Offset; 4]);

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
