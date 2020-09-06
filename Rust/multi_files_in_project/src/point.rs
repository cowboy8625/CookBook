use std::ops::{AddAssign, SubAssign};

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn add(&mut self, x:T, y:T) where T: AddAssign {
        self.x += x;
        self.y += y;
    }
    pub fn sub(&mut self, x:T, y:T) where T: SubAssign {
        self.x -= x;
        self.y -= y;
    }
}
