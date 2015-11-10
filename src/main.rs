mod buffer;
use buffer::Buffer;
use buffer::Coord;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_buffer_insert() {
    let mut buffer = Buffer::new();
    buffer.insert_string_at_coord("aí", Coord::new(0, 0));
    assert!(&(buffer.contents[0]) == "aí\n");
    buffer.insert_string_at_coord("éå", Coord::new(1, 0));
    assert!(&(buffer.contents[0]) == "aéåí\n");
    buffer.insert_string_at_coord("éå", Coord::new(2, 0));
    assert!(&(buffer.contents[0]) == "aééååí\n");

    let mut buffer1 = Buffer::new();
    buffer1.insert_string_at_coord("ab", Coord::new(0, 0));
    assert!(&(buffer1.contents[0]) == "ab\n");
    buffer1.insert_string_at_coord("c\n", Coord::new(1, 0));
    assert!(buffer1.contents.len() == 2);
    assert!(&(buffer1.contents[0]) == "ac\n");
    assert!(&(buffer1.contents[1]) == "b\n");

    let mut buffer2 = Buffer::new();
    buffer2.insert_string_at_coord("áb", Coord::new(0, 0));
    buffer2.insert_string_at_coord("cd\néf", Coord::new(1, 0));
    assert!(buffer2.contents.len() == 2);
    assert!(&(buffer2.contents[0]) == "ácd\n");
    assert!(&(buffer2.contents[1]) == "éfb\n");
    buffer2.insert_string_at_coord("gh\nîj\n", Coord::new(2, 1));
    assert!(buffer2.contents.len() == 4);
    for line in buffer2.contents.iter() {
        println!("{}", line);
    }
    assert!(&(buffer2.contents[0]) == "ácd\n");
    assert!(&(buffer2.contents[1]) == "éfgh\n");
    assert!(&(buffer2.contents[2]) == "îj\n");
    assert!(&(buffer2.contents[3]) == "b\n");
}
