use std::cmp::Ordering;
extern crate serde_macros;


#[derive(Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyEvent {
    pub key_char: i32,
    pub modifier_flags: i32
}

#[derive(Debug)]
pub enum KeyCode {
    UpArrowFunctionKey,    // 0xF700
    DownArrowFunctionKey,  // 0xF701
    LeftArrowFunctionKey,  // 0xF702
    RightArrowFunctionKey  // 0xF703
}

pub fn key_code_from_i32(key_code: i32) -> Option<KeyCode> {
    match key_code {
        0xF700 => Some(KeyCode::UpArrowFunctionKey),
        0xF701 => Some(KeyCode::DownArrowFunctionKey),
        0xF702 => Some(KeyCode::LeftArrowFunctionKey),
        0xF703 => Some(KeyCode::RightArrowFunctionKey),
        _ => None
    }
}
