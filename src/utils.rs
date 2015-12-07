use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        }
        else if self.y > other.y {
            Some(Ordering::Greater)
        }
        else if self.x > other.x {
            Some(Ordering::Greater)
        }
        else {
            Some(Ordering::Less)
        }
    }
}
