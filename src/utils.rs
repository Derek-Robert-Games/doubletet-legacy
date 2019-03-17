use components as c;
use settings;

#[derive(Debug)]
pub struct Offset {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    pub fn get_position(&self) -> c::Position {
        c::Position {
            x: (self.x as f64) * settings::RECT_WIDTH,
            y: (self.y as f64) * settings::RECT_HEIGHT,
        }
    }
}
