// -*- flycheck-rust-crate-type: "bin" -*-

use buffer::{Buffer};
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::result::Result;
use std::io;
use utils::{Direction, Coord};

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
}

pub trait Actionable {
    fn perform(&self);
}

pub struct MoveAction<'a> {
    pub editor: &'a Editor,
    pub direction: Direction,
    pub units: i32,
}

impl<'a> MoveAction<'a> {
    pub fn new(editor: &Editor, direction: Direction, units: i32) -> MoveAction {
        MoveAction {
            editor: editor,
            direction: direction,
            units: units,
        }
    }

    pub fn perform(&self) {
        // self.editor.current_buffer().
    }
}

impl<'a> Actionable for MoveAction<'a> {
    fn perform(&self) {
        println!("Perform MoveAction");
    }
}
