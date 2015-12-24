use std::cmp::Ordering;
use buffer::Buffer;
extern crate serde_macros;


#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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
    pub message_type: MessageType,
    pub key_char: i32,
    pub modifier_flags: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferStateRequest {
    pub message_type: MessageType,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferStateResponse {
    pub message_type: MessageType,
    pub buffer: Buffer,
}

impl Message for KeyEvent {
    fn message_type(&self) -> MessageType {
        self.message_type.clone()
    }
}

impl Message for BufferStateRequest {
    fn message_type(&self) -> MessageType {
        self.message_type.clone()
    }
}

impl Message for BufferStateResponse {
    fn message_type(&self) -> MessageType {
        self.message_type.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    BufferStateRequest,
    BufferStateResponse,
    ActionResult,
    KeyEvent,
}

pub trait Message {
    fn message_type(&self) -> MessageType;
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
