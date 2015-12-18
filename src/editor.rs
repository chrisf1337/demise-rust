// -*- flycheck-rust-crate-type: "bin" -*-

use buffer::{Buffer};
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::result::Result;
use std::io;
use std::fmt;
use utils::{Direction, Coord, KeyEvent, key_code_from_i32, KeyCode};

#[derive(Debug)]
pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub current_buffer_index: usize
}

impl<'a> Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![Buffer::new()],
            current_buffer_index: 0
        }
    }

    pub fn current_buffer(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
    }

    pub fn open_file(&mut self, buffer_index: usize) -> io::Result<()> {
        let buffer = &mut self.buffers[buffer_index];
        let mut file = PathBuf::from("/Users/chrisf/projects/demise-rust");
        file.push("test/test.txt");
        println!("{:?}", file);
        let mut f = try!(File::open(file));
        let mut contents: String = "".to_string();
        try!(f.read_to_string(&mut contents));
        for (index, line) in contents.lines().enumerate() {
            buffer.insert_string_at_coord(line, &Coord::new(0, index));
        }
        Ok(())
    }

    fn move_action_for_key_event(&mut self, key_event: &KeyEvent) -> Option<MoveAction> {
        let key_code = key_code_from_i32(key_event.key_char);
        match key_code {
            Some(k) => match k {
                KeyCode::UpArrowFunctionKey => {
                    Some(MoveAction::new(self, Direction::Up, 1))
                },
                KeyCode::DownArrowFunctionKey => {
                    Some(MoveAction::new(self, Direction::Down, 1))
                },
                KeyCode::LeftArrowFunctionKey => {
                    Some(MoveAction::new(self, Direction::Left, 1))
                },
                KeyCode::RightArrowFunctionKey => {
                    Some(MoveAction::new(self, Direction::Right, 1))
                }
            },
            None => {
                println!("Key code not recognized.");
                None
            }
        }
    }

    pub fn perform_action_for_key_event(&mut self, key_event: &KeyEvent) -> ActionResult {
        {
            let move_action = self.move_action_for_key_event(key_event);
            if move_action.is_some() {
                return move_action.unwrap().perform()
            }
        }
        ActionResult {
            change_types: vec![],
            new_point: self.current_buffer().point(),
            lines_changed_after_line: 0,
            lines_changed: vec![]
        }
    }
}

pub trait Actionable {
    fn perform(&mut self) -> ActionResult;
}

#[derive(Debug)]
pub enum ChangeType {
    NoChange,
    PointChanged,
    LinesChanged
}

#[derive(Debug)]
pub struct ActionResult {
    pub change_types: Vec<ChangeType>,
    pub new_point: Coord,
    pub lines_changed_after_line: usize,
    pub lines_changed: Vec<usize>
}

#[derive(Debug)]
pub struct MoveAction<'a> {
    pub editor: &'a mut Editor,
    pub direction: Direction,
    pub units: i32,
}

impl<'a> MoveAction<'a> {
    pub fn new(editor: &mut Editor, direction: Direction, units: i32) -> MoveAction {
        MoveAction {
            editor: editor,
            direction: direction,
            units: units,
        }
    }
}

impl<'a> fmt::Display for MoveAction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoveAction {{ direction: {:?}, units: {} }}", self.direction, self.units)
    }
}

impl<'a> Actionable for MoveAction<'a> {
    fn perform(&mut self) -> ActionResult {
        println!("Perform {}", self);
        let new_point = self.editor.current_buffer().move_point_in_dir(&self.direction, self.units);
        ActionResult {
            change_types: vec![ChangeType::PointChanged],
            new_point: new_point,
            lines_changed_after_line: 0,
            lines_changed: vec![]
        }
    }
}
