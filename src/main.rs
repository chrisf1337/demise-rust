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
    buffer.insert_string_at_coord("éå", Coord::new(0, 0));
    assert!(&(buffer.contents[0]) == "éåaí\n");
}
