use crate::vec2::Vec2;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    #[inline]
    pub fn rotate_ccw(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    #[inline]
    pub fn rotate_cw(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    #[inline]
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }

    #[inline]
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Direction::Right => Vec2::right(),
            Direction::Up => Vec2::up(),
            Direction::Left => Vec2::left(),
            Direction::Down => Vec2::down(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Left => write!(f, "Left"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

#[derive(Debug)]
pub struct ParseDirectionError(String);

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right" => Ok(Direction::Right),
            "up" => Ok(Direction::Up),
            "left" => Ok(Direction::Left),
            "down" => Ok(Direction::Down),
            _ => Err(ParseDirectionError(s.to_string())),
        }
    }
}
