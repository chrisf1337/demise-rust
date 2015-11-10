pub struct Buffer {
    pub contents: Vec<String>,
    pub end: Coord,
}

pub struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: vec!["\n".to_string()],
            end: Coord::new(1, 0)
        }
    }

    pub fn insert_string_at_coord(&mut self, string: &str, coord: Coord) {
        if string.len() == 0 {
            return;
        }
        assert!(coord.y <= self.contents.len());
        let line = &mut self.contents[coord.y];
        let char_indices: Vec<(usize, char)> = line.char_indices().collect();
        for (index, _) in char_indices {
            if coord.x == index {
                for (string_index, string_char) in string.char_indices() {
                    line.insert(index + string_index, string_char);
                }
            }
        }
    }
}
