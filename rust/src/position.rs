use super::*;

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(ix: i32, iy: i32) -> Self {
        Self { x: ix, y: iy }
    }

    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    pub fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }
}