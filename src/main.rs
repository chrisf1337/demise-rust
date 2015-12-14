#![feature(drain)]
extern crate byteorder;
extern crate serde_json;

mod buffer;
mod editor;
mod utils;
use buffer::{Buffer};
use editor::{Editor, MoveAction, Actionable};
use utils::{Direction, Coord};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Cursor};
use byteorder::*;
use serde_json::Value;

const PACKET_SIZE_BYTES: usize = 4;

fn handle_client(mut stream: TcpStream) {
    let mut buf: Vec<u8>;
    loop {
        let mut size_buf = [0u8; PACKET_SIZE_BYTES];
        let size: u32;

        let _ = match stream.read(&mut size_buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        println!("{:?}", size_buf);
        size = Cursor::new(&size_buf).read_u32::<LittleEndian>().unwrap();
        println!("{}", size);

        buf = vec![0; size as usize];
        let _ = match stream.read(&mut buf[..]) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        let string = String::from_utf8(buf).unwrap();
        println!("{:?}", string);
        let json: Value = serde_json::from_str(&string).unwrap();
        println!("{:?}", json);

        // match stream.write(&buf) {
        //     Err(_) => break,
        //     Ok(_) => continue
        // }
    }
}

fn open_socket() {
    let listener = TcpListener::bind("localhost:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                println!("Socket error: {}", e);
            }
            Ok(stream) => {
                println!("Spawning thread");
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}

fn main() {
    open_socket();
}

#[test]
fn test_buffer_insert() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_coord("aí", &Coord::new(0, 0));
    assert_eq!(buffer.chars, 3);
    assert_eq!(buffer.bytes, 4);
    assert_eq!(buffer.contents[0], "aí\n");
    buffer.insert_string_at_coord("éå", &Coord::new(1, 0));
    assert_eq!(buffer.chars, 5);
    assert_eq!(buffer.contents[0], "aéåí\n");
    buffer.insert_string_at_coord("éå", &Coord::new(2, 0));
    assert_eq!(buffer.chars, 7);
    assert_eq!(buffer.contents[0], "aééååí\n");

    let mut buffer1 = Buffer::new();
    buffer1.insert_string_at_coord("ab", &Coord::new(0, 0));
    assert_eq!(buffer1.contents[0], "ab\n");
    buffer1.insert_string_at_coord("c\n", &Coord::new(1, 0));
    assert_eq!(buffer1.contents.len(), 2);
    assert_eq!(buffer1.contents[0], "ac\n");
    assert_eq!(buffer1.contents[1], "b\n");

    let mut buffer2 = Buffer::new();
    buffer2.insert_string_at_coord("áb", &Coord::new(0, 0));
    buffer2.insert_string_at_coord("cd\néf", &Coord::new(1, 0));
    assert_eq!(buffer2.chars, 8);
    assert_eq!(buffer2.contents.len(), 2);
    assert_eq!(buffer2.contents[0], "ácd\n");
    assert_eq!(buffer2.contents[1], "éfb\n");
    buffer2.insert_string_at_coord("gh\nîj\n", &Coord::new(2, 1));
    assert_eq!(buffer2.contents.len(), 4);
    for line in buffer2.contents.iter() {
        println!("{}", line);
    }
    assert_eq!(buffer2.contents[0], "ácd\n");
    assert_eq!(buffer2.contents[1], "éfgh\n");
    assert_eq!(buffer2.contents[2], "îj\n");
    assert_eq!(buffer2.contents[3], "b\n");

    let mut buffer3 = Buffer::new();
    buffer3.insert_string_at_point("\n\n");
    assert_eq!(buffer3.contents.len(), 3);
    assert_eq!(buffer3.contents[0], "\n");
    assert_eq!(buffer3.contents[1], "\n");
    assert_eq!(buffer3.contents[2], "\n");

    let mut buffer4 = Buffer::new();
    buffer4.insert_string_at_coord("abc", &Coord::new(0, 1));
    assert_eq!(buffer4.contents[0], "\n");
    assert_eq!(buffer4.contents[1], "abc\n");
}

#[test]
fn test_buffer_move_point_dist() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_point("abc");
    assert_eq!(buffer.contents[0], "abc\n");
    assert_eq!(buffer.point(), Coord::new(0, 0));
    buffer.move_point_dist(3);
    assert_eq!(buffer.point(), Coord::new(3, 0));
    buffer.move_point_dist(1);
    assert_eq!(buffer.point(), Coord::new(3, 0));
    buffer.insert_string_at_point_and_move("def\ngh");
    assert_eq!(buffer.contents[0], "abcdef\n");
    assert_eq!(buffer.contents[1], "gh\n");
    assert_eq!(buffer.point(), Coord::new(2, 1));
    buffer.move_point_dist(-5);
    assert_eq!(buffer.point(), Coord::new(4, 0));
    buffer.move_point_dist(-5);
    assert_eq!(buffer.point(), Coord::new(0, 0));
    buffer.move_point_dist(20);
    assert_eq!(buffer.point(), Coord::new(2, 1));
    buffer.insert_string_at_point_and_move("\n\n\n");
    assert_eq!(buffer.point(), Coord::new(0, 4));
    buffer.move_point_dist(-3);
    assert_eq!(buffer.point(), Coord::new(2, 1));
}

#[test]
fn test_buffer_move_point_in_dir() {
    let mut buffer = Buffer::new();
    // abcdef\n
    // def\n
    // ghidefdef\n
    // a\n
    buffer.insert_string_at_point("abcdef\ndef\nghidefdef\na");
    assert_eq!(buffer.contents.len(), 4);
    buffer.move_point_in_dir(Direction::Down, 3);
    assert_eq!(buffer.point(), Coord::new(0, 3));
    buffer.move_point_in_dir(Direction::Up, 2);
    assert_eq!(buffer.point(), Coord::new(0, 1));
    buffer.move_point_in_dir(Direction::Up, 3);
    assert_eq!(buffer.point(), Coord::new(0, 0));
    buffer.move_point_in_dir(Direction::Down, 3);
    assert_eq!(buffer.point(), Coord::new(0, 3));
    buffer.move_point_in_dir(Direction::Down, 3);
    assert_eq!(buffer.point(), Coord::new(0, 4));

    buffer.set_point(&Coord::new(0, 0));
    assert_eq!(buffer.point(), Coord::new(0, 0));
    buffer.move_point_in_dir(Direction::Right, 6);
    assert_eq!(buffer.point(), Coord::new(6, 0));
    buffer.move_point_in_dir(Direction::Down, 1);
    assert_eq!(buffer.point(), Coord::new(3, 1));
    buffer.move_point_in_dir(Direction::Left, 1);
    assert_eq!(buffer.point(), Coord::new(2, 1));
    buffer.move_point_in_dir(Direction::Up, 1);
    assert_eq!(buffer.point(), Coord::new(2, 0));
    buffer.move_point_in_dir(Direction::Down, 3);
    assert_eq!(buffer.point(), Coord::new(1, 3));
    buffer.move_point_in_dir(Direction::Down, 1);
    assert_eq!(buffer.point(), Coord::new(0, 4));
}

#[test]
fn test_buffer_delete_from_to() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_point("ðïc\nåâf");
    assert_eq!(buffer.contents[0], "ðïc\n");
    assert_eq!(buffer.contents[1], "åâf\n");
    buffer.delete_from_to(&Coord::new(0, 0), &Coord::new(4, 0));
    assert_eq!(buffer.contents.len(), 1);
    assert_eq!(buffer.contents[0], "åâf\n");

    let mut buffer1 = Buffer::new();
    buffer1.insert_string_at_point("abc\ndef");
    buffer1.delete_from_to(&Coord::new(2, 0), &Coord::new(4, 0));
    assert_eq!(buffer1.contents.len(), 1);
    assert_eq!(buffer1.contents[0], "abdef\n");
    buffer1.insert_string_at_coord("hij\n", &Coord::new(0, 1));
    buffer1.delete_from_to(&Coord::new(2, 1), &Coord::new(4, 1));
    assert_eq!(buffer1.contents[1], "hi\n");

    let mut buffer2 = Buffer::new();
    buffer2.insert_string_at_point("abc\ndef");
    buffer2.delete_from_to(&Coord::new(1, 1), &Coord::new(3, 1));
    assert_eq!(buffer2.contents[0], "abc\n");
    assert_eq!(buffer2.contents[1], "d\n");
    buffer2.insert_string_at_coord("ef", &Coord::new(1, 1));
    assert_eq!(buffer2.contents[0], "abc\n");
    assert_eq!(buffer2.contents[1], "def\n");
    buffer2.delete_from_to(&Coord::new(1, 0), &Coord::new(1, 1));
    assert_eq!(buffer2.contents.len(), 1);
    assert_eq!(buffer2.contents[0], "aef\n");
    buffer2.insert_string_at_coord("bcd\n", &Coord::new(0, 0));
    assert_eq!(buffer2.contents[0], "bcd\n");
    assert_eq!(buffer2.contents[1], "aef\n");
    buffer2.delete_from_to(&Coord::new(2, 0), &Coord::new(3, 1));
    assert_eq!(buffer2.contents.len(), 1);
    assert_eq!(buffer2.contents[0], "bc\n");
    buffer2.insert_string_at_coord("abc\ndef\nghi", &Coord::new(0, 0));
    assert_eq!(buffer2.contents.len(), 3);
    assert_eq!(buffer2.contents[0], "abc\n");
    assert_eq!(buffer2.contents[1], "def\n");
    assert_eq!(buffer2.contents[2], "ghibc\n");
    buffer2.delete_from_to(&Coord::new(0, 0), &Coord::new(2, 2));
    assert_eq!(buffer2.contents.len(), 1);
    assert_eq!(buffer2.contents[0], "ibc\n");
    buffer2.insert_string_at_coord("abc\ndef\nghi", &Coord::new(0, 0));
    buffer2.delete_from_to(&Coord::new(0, 0), &Coord::new(7, 2));
    assert_eq!(buffer2.contents.len(), 1);
    assert_eq!(buffer2.contents[0], "\n");
}

#[test]
fn test_editor() {
    let mut editor = Editor::new();
    {
        let current_buffer = editor.current_buffer();
        current_buffer.insert_string_at_point("abc");
    }
    assert_eq!(editor.buffers[0].contents[0], "abc\n");

    let mut editor1 = Editor::new();
    assert_eq!(editor1.open_file(0).is_ok(), true);
    let buffer1 = editor1.current_buffer();
    assert_eq!(buffer1.contents.len(), 2);
    assert_eq!(buffer1.contents[0], "abcdef\n");
    assert_eq!(buffer1.contents[1], "ghi\n");
}

#[test]
fn test_editor_actions() {
    let editor = Editor::new();
    let move_action = MoveAction::new(&editor, Direction::Up, 0);
}
