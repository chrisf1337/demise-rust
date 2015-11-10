mod buffer;
use buffer::Buffer;
use buffer::Coord;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_buffer_insert() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_coord("aí", &Coord::new(0, 0));
    assert_eq!(buffer.chars, 3);
    assert_eq!(buffer.bytes, 4);
    assert_eq!(&(buffer.contents[0]), "aí\n");
    buffer.insert_string_at_coord("éå", &Coord::new(1, 0));
    assert_eq!(buffer.chars, 5);
    assert_eq!(&(buffer.contents[0]), "aéåí\n");
    buffer.insert_string_at_coord("éå", &Coord::new(2, 0));
    assert_eq!(buffer.chars, 7);
    assert_eq!(&(buffer.contents[0]), "aééååí\n");

    let mut buffer1 = Buffer::new();
    buffer1.insert_string_at_coord("ab", &Coord::new(0, 0));
    assert_eq!(&(buffer1.contents[0]), "ab\n");
    buffer1.insert_string_at_coord("c\n", &Coord::new(1, 0));
    assert_eq!(buffer1.contents.len(), 2);
    assert_eq!(&(buffer1.contents[0]), "ac\n");
    assert_eq!(&(buffer1.contents[1]), "b\n");

    let mut buffer2 = Buffer::new();
    buffer2.insert_string_at_coord("áb", &Coord::new(0, 0));
    buffer2.insert_string_at_coord("cd\néf", &Coord::new(1, 0));
    assert_eq!(buffer2.chars, 8);
    assert_eq!(buffer2.contents.len(), 2);
    assert_eq!(&(buffer2.contents[0]), "ácd\n");
    assert_eq!(&(buffer2.contents[1]), "éfb\n");
    buffer2.insert_string_at_coord("gh\nîj\n", &Coord::new(2, 1));
    assert_eq!(buffer2.contents.len(), 4);
    for line in buffer2.contents.iter() {
        println!("{}", line);
    }
    assert_eq!(&(buffer2.contents[0]), "ácd\n");
    assert_eq!(&(buffer2.contents[1]), "éfgh\n");
    assert_eq!(&(buffer2.contents[2]), "îj\n");
    assert_eq!(&(buffer2.contents[3]), "b\n");

    let mut buffer3 = Buffer::new();
    buffer3.insert_string_at_point("\n\n");
    assert_eq!(buffer3.contents.len(), 3);
    assert_eq!(buffer3.contents[0], "\n");
    assert_eq!(buffer3.contents[1], "\n");
    assert_eq!(buffer3.contents[2], "\n");

}

#[test]
fn test_buffer_move_point() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_point("abc");
    assert_eq!(buffer.contents[0], "abc\n");
    assert_eq!(buffer.point, Coord::new(3, 0));
    buffer.move_point(1);
    assert_eq!(buffer.point, Coord::new(3, 0));
    buffer.insert_string_at_point("def\ngh");
    assert_eq!(buffer.contents[0], "abcdef\n");
    assert_eq!(buffer.contents[1], "gh\n");
    assert_eq!(buffer.point, Coord::new(2, 1));
    buffer.move_point(-5);
    assert_eq!(buffer.point, Coord::new(4, 0));
    buffer.move_point(-5);
    assert_eq!(buffer.point, Coord::new(0, 0));
    buffer.move_point(20);
    assert_eq!(buffer.point, Coord::new(2, 1));
    buffer.insert_string_at_point("\n\n\n");
    assert_eq!(buffer.point, Coord::new(0, 4));
    buffer.move_point(-3);
    assert_eq!(buffer.point, Coord::new(2, 1));
}
