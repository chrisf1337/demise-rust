#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate byteorder;
extern crate serde_json;
extern crate serde;
extern crate serde_macros;

pub mod buffer;
pub mod editor;
pub mod utils;
use editor::{Editor};
use utils::{KeyEvent, BufferStateRequest, BufferStateResponse, MessageType};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::{Cursor};
use byteorder::*;
use serde_json::value::{Value};
use serde::ser::Serialize;

const PACKET_SIZE_BYTES: usize = 4;

fn handle_client(mut stream: TcpStream) {
    let mut editor = Editor::new();
    let _ = editor.open_file(0);
    let mut recv_buf: Vec<u8>;
    loop {
        let mut recv_size_buf = [0u8; PACKET_SIZE_BYTES];
        let recv_size: u32;

        let _ = match stream.read(&mut recv_size_buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        println!("{:?}", recv_size_buf);
        recv_size = match Cursor::new(&recv_size_buf).read_u32::<LittleEndian>() {
            Ok(size) => size,
            Err(e) => {
                println!("Size conversion error: {}", e);
                continue;
            }
        };
        println!("{}", recv_size);

        recv_buf = vec![0; recv_size as usize];
        let _ = match stream.read(&mut recv_buf[..]) {
            Err(e) => {
                println!("Stream read error: {}", e);
                continue;
            },
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        let string = match String::from_utf8(recv_buf) {
            Ok(s) => s,
            Err(e) => {
                println!("String conversion error: {}", e);
                continue;
            }
        };
        let recv_value: Value = match serde_json::from_str(&string) {
            Ok(rv) => rv,
            Err(e) => {
                println!("Serde error: {}", e);
                continue;
            }
        };
        let message_type_object = match recv_value.lookup("message_type") {
            Some(mt) => match mt.as_object() {
                Some(mtv) => mtv,
                None => {
                    println!("Not an object");
                    continue;
                }
            },
            None => {
                println!("No message_type element");
                continue;
            }
        };

        let message_type: Vec<_> = message_type_object.keys().cloned().collect();
        assert_eq!(message_type.len(), 1);
        if message_type[0] == "KeyEvent" {
            let recv_json: KeyEvent = match serde_json::from_str(&string) {
                Ok(rj) => rj,
                Err(e) => {
                    println!("KeyEvent deserialization error: {}", e);
                    continue;
                }
            };
            println!("{:?}", recv_json);
            let action_result = editor.perform_action_for_key_event(&recv_json);
            println!("{:?}", action_result);

            match send_message(&mut stream, &action_result) {
                Err(e) => println!("Send message error: {}", e),
                Ok(()) => {}
            }
        } else if message_type[0] == "BufferStateRequest" {
            println!("Buffer state request received");
            let recv_json: BufferStateRequest = match serde_json::from_str(&string) {
                Ok(rj) => rj,
                Err(e) => {
                    println!("BufferStateRequest deserialization error: {}", e);
                    continue;
                }
            };
            let buffer = editor.buffers[recv_json.index].clone();
            let buffer_state_response = BufferStateResponse {
                message_type: MessageType::BufferStateResponse,
                buffer: buffer
            };
            match send_message(&mut stream, &buffer_state_response) {
                Ok(()) => {},
                Err(e) => println!("Send message error: {}", e),
            }
        }
    }
}

fn send_message<T: Serialize>(stream: &mut TcpStream, message: &T) -> Result<()> {
    let mut send_size_vec = vec![];
    let send_result = serde_json::to_vec(&message);
    let send_json = send_result.unwrap();
    let send_size = send_json.len() as u32;
    send_size_vec.write_u32::<LittleEndian>(send_size).unwrap();
    println!("{:?}", send_size_vec);
    try!(stream.write(&send_size_vec[..]));
    try!(stream.write(&send_json[..]));
    Ok(())
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

#[cfg(test)]
mod test {
    use buffer::Buffer;
    use editor::Editor;
    use utils::{Direction, Coord, MessageType, KeyEvent};
    use std::io::prelude::*;

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
        buffer.move_point_in_dir(&Direction::Down, 3);
        assert_eq!(buffer.point(), Coord::new(0, 3));
        buffer.move_point_in_dir(&Direction::Up, 2);
        assert_eq!(buffer.point(), Coord::new(0, 1));
        buffer.move_point_in_dir(&Direction::Up, 3);
        assert_eq!(buffer.point(), Coord::new(0, 0));
        buffer.move_point_in_dir(&Direction::Down, 3);
        assert_eq!(buffer.point(), Coord::new(0, 3));
        buffer.move_point_in_dir(&Direction::Down, 3);
        assert_eq!(buffer.point(), Coord::new(0, 4));

        buffer.set_point(&Coord::new(0, 0));
        assert_eq!(buffer.point(), Coord::new(0, 0));
        buffer.move_point_in_dir(&Direction::Right, 6);
        assert_eq!(buffer.point(), Coord::new(6, 0));
        buffer.move_point_in_dir(&Direction::Down, 1);
        assert_eq!(buffer.point(), Coord::new(3, 1));
        buffer.move_point_in_dir(&Direction::Left, 1);
        assert_eq!(buffer.point(), Coord::new(2, 1));
        buffer.move_point_in_dir(&Direction::Up, 1);
        assert_eq!(buffer.point(), Coord::new(2, 0));
        buffer.move_point_in_dir(&Direction::Down, 3);
        assert_eq!(buffer.point(), Coord::new(1, 3));
        buffer.move_point_in_dir(&Direction::Down, 1);
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
        let mut editor = Editor::new();
        let _ = editor.open_file(0);
        editor.perform_action_for_key_event(&KeyEvent { message_type: MessageType::KeyEvent, key_char: 0xF703, modifier_flags: 0 });
        assert_eq!(editor.current_buffer().point(), Coord::new(1, 0));
        editor.perform_action_for_key_event(&KeyEvent { message_type: MessageType::KeyEvent, key_char: 0xF701, modifier_flags: 0 });
        assert_eq!(editor.current_buffer().point(), Coord::new(1, 1));
    }

    #[test]
    fn test_serialize() {

    }
}
