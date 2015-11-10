extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation;

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
        if string.is_empty() {
            return;
        }

        assert!(coord.y <= self.contents.len());
        assert!(coord.x < self.contents[coord.y].len());

        let lines: Vec<&str> = string.lines().collect();
        // Only one line; check if line ends in \n
        if lines.len() == 1 {
            let last_char = string.chars().last();
            if last_char == Some('\n') {
                let line = self.contents[coord.y].clone();
                // let char_indices: Vec<(usize, char)>;
                let char_indices: Vec<(usize, &str)> = UnicodeSegmentation::grapheme_indices(&line[..], true).collect();
                // {
                    // let line = &(self.contents[coord.y]);
                    // char_indices = line.char_indices().collect();
                // }
                for (i, &(index, _)) in char_indices.iter().enumerate() {
                    if coord.x == i {
                        let mut first_half_string: String;
                        let second_half_string: String;
                        {
                            let (first_half, second_half) = self.contents[coord.y].split_at(index);
                            first_half_string = first_half.to_string();
                            first_half_string.push_str(string);
                            second_half_string = second_half.to_string();
                        }
                        self.contents[coord.y] = first_half_string;
                        self.contents.insert(coord.y + 1, second_half_string);
                    }
                }
            }
            else {
                let line_clone = self.contents[coord.y].clone();
                let line = &mut self.contents[coord.y];
                // let line_str: &str = &line;
                // let char_indices: Vec<(usize, char)> = line.char_indices().collect();
                let char_indices: Vec<(usize, &str)> = UnicodeSegmentation::grapheme_indices(&line_clone[..], true).collect();
                for (i, &(index, _)) in char_indices.iter().enumerate() {
                    if coord.x == i {
                        for (string_index, string_char) in string.char_indices() {
                            line.insert(index + string_index, string_char);
                        }
                    }
                }
            }
        }
        else {
            let mut current_line_number = coord.y;
            let mut second_half_string = "".to_string();
            let line = self.contents[coord.y].clone();
            // let char_indices: Vec<(usize, char)> = self.contents[current_line_number].char_indices().collect();
            let char_indices: Vec<(usize, &str)> = UnicodeSegmentation::grapheme_indices(&line[..], true).collect();
            for (i, &(index, _)) in char_indices.iter().enumerate() {
                if coord.x == i {
                    let mut first_half_string: String;
                    {
                        let (first_half, second_half) = self.contents[coord.y].split_at(index);
                        first_half_string = first_half.to_string();
                        first_half_string.push_str(&(lines[0].to_string() + "\n"));
                        second_half_string = second_half.to_string();
                    }
                    self.contents[coord.y] = first_half_string;
                }
            }
            current_line_number += 1;
            for line in lines.iter().take(lines.len() - 1).skip(1) {
                self.contents.insert(current_line_number, line.to_string() + "\n");
                current_line_number += 1;
            }
            let mut last_line = lines.iter().last().unwrap().to_string();
            let last_char = string.chars().last();
            if last_char == Some('\n') {
                self.contents.insert(current_line_number, last_line.to_string() + "\n");
                self.contents.insert(current_line_number + 1, second_half_string);
            }
            else {
                last_line.push_str(&second_half_string);
                self.contents.insert(current_line_number, last_line);
            }
        }
    }
}
