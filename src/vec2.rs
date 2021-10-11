use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn right() -> Vec2 {
        Vec2 { x: 1, y: 0 }
    }

    pub fn up() -> Vec2 {
        Vec2 { x: 0, y: 1 }
    }

    pub fn left() -> Vec2 {
        Vec2 { x: -1, y: 0 }
    }

    pub fn down() -> Vec2 {
        Vec2 { x: 0, y: -1 }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn abs(self) -> Vec2 {
        Vec2 { x: self.x.abs(), y: self.y.abs() }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = *self - other;
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: i32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}
