extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation as UniSeg;
use editor::Direction;
use std::cmp::Ordering;

pub struct Buffer {
    pub contents: Vec<String>,
    pub bytes: usize,
    pub chars: usize,
    point: Coord
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: vec!["\n".to_string()],
            bytes: 1,
            chars: 1,
            point: Coord::new(0, 0)
        }
    }

    /**
     * Moves self.point if the insert point is before the current point.
     */
    pub fn insert_string_at_coord(&mut self, string: &str, coord: &Coord) {
        if string.is_empty() {
            return;
        }

        self.bytes += string.bytes().count();
        self.chars += string.chars().count();

        assert!(coord.y <= self.contents.len());
        if coord.y == self.contents.len() {
            self.contents.push("\n".to_string());
        }
        assert!(coord.x < self.contents[coord.y].len());

        if string.contains('\n') {
            let lines: Vec<&str> = string.lines().collect();

            let mut current_line_number = coord.y;
            let mut second_half_string = "".to_string();
            let line = self.contents[coord.y].clone();
            // let char_indices: Vec<(usize, char)> = self.contents[current_line_number].char_indices().collect();
            let char_indices: Vec<(usize, &str)> =
                UniSeg::grapheme_indices(&line[..], true).collect();
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

            if lines.len() == 1 {
                self.contents.insert(current_line_number, second_half_string);
            }
            else {
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
        else {
            let line_clone = self.contents[coord.y].clone();
            let line = &mut self.contents[coord.y];
            // let line_str: &str = &line;
            // let char_indices: Vec<(usize, char)> = line.char_indices().collect();
            let char_indices: Vec<(usize, &str)> =
                UniSeg::grapheme_indices(&line_clone[..], true).collect();
            for (i, &(index, _)) in char_indices.iter().enumerate() {
                if coord.x == i {
                    for (string_index, string_char) in string.char_indices() {
                        line.insert(index + string_index, string_char);
                    }
                }
            }
        }

        // Move point if insert point is less than current point
        if coord < &self.point {
            self.move_point_dist(UniSeg::grapheme_indices(string, true).count() as i32);
        }
    }

    // Does not move self.point
    pub fn insert_string_at_point(&mut self, string: &str) {
        let point = self.point.clone();
        self.insert_string_at_coord(string, &point);
    }

    pub fn insert_string_at_point_and_move(&mut self, string: &str) {
        let point = self.point.clone();
        self.insert_string_at_coord(string, &point);
        self.move_point_dist(UniSeg::grapheme_indices(string, true).count() as i32);
    }

    pub fn move_point_dist(&mut self, distance: i32) -> Coord {
        if distance == 0 {
            return self.point.clone();
        }
        let mut dist: i32;
        if distance < 0 {
            dist = -distance;
            while dist > 0 {
                if self.point.x == 0 {
                    if self.point.y == 0 {
                        self.point = Coord::new(0, 0);
                        return Coord::new(0, 0);
                    }
                    self.point.y -= 1;
                    self.point.x = self.contents[self.point.y].len() - 1;
                }
                else {
                    self.point.x -= 1;
                }
                dist -= 1;
            }
        }
        else {
            dist = distance;
            while dist > 0 {
                self.point.x += 1;
                if self.point.x == self.contents[self.point.y].len() {
                    if self.point.y == self.contents.len() - 1 {
                        self.point = Coord::new(self.contents[self.point.y].len() - 1,
                                                self.contents.len() - 1);
                        return self.point.clone();
                    }
                    self.point.y += 1;
                    self.point.x = 0;
                }
                dist -= 1;
            }
        }
        return self.point.clone();
    }

    pub fn move_point_in_dir(&mut self, direction: Direction, units: i32) -> Coord {
        if units == 0 {
            return self.point.clone();
        }
        match direction {
            Direction::Up => {
                let mut offset: i32;
                if units < 0 {
                    offset = -units;
                }
                else {
                    offset = units;
                }
                while offset > 0 && self.point.y > 0 {
                    self.point.y -= 1;
                    offset -= 1;
                }
            },
            Direction::Down => {
                let mut offset: i32;
                if units < 0 {
                    offset = -units;
                }
                else {
                    offset = units;
                }
                while offset > 0 && self.point.y < self.contents.len() {
                    self.point.y += 1;
                    offset -= 1;
                }
            },
            Direction::Left => {

            },
            Direction::Right => {

            },
        }
        return self.point.clone();
    }

    pub fn delete_from_to(&mut self, start: &Coord, end: &Coord) {
        assert!(start.y < self.contents.len() && end.y < self.contents.len());
        assert!(start.x <= self.contents[start.y].len() && end.x <= self.contents[end.y].len());
        assert!(start <= end);
        if start == end {
            return;
        }

        // Single line
        if start.y == end.y {
            let current_line = self.contents[start.y].clone();
            let char_indices: Vec<(usize, &str)> =
                UniSeg::grapheme_indices(&current_line[..], true).collect();
            println!("{:?}", char_indices);
            let (last_grapheme_index, _) = *(char_indices.iter().last().unwrap());
            println!("Last grapheme index: {}", last_grapheme_index);
            if end.x == char_indices.iter().count() {
                // Removed newline as well, so concatenate with following line if it exists
                if start.y != self.contents.len() - 1 {
                    let (start_index, _) = *(char_indices.iter().nth(start.x).unwrap());
                    let end_index = current_line.len();
                    self.contents[start.y].drain(start_index..end_index);
                    println!("After drain: {}", self.contents[start.y]);
                    if self.contents[start.y].is_empty() {
                        self.contents.remove(start.y);
                    }
                    else {
                        let next_line = self.contents[start.y + 1].clone();
                        self.contents[start.y].push_str(&next_line);
                        self.contents.remove(start.y + 1);
                    }
                }
                else {
                    // Following line does not exist, so keep newline
                    let (start_index, _) = *(char_indices.iter().nth(start.x).unwrap());
                    let end_index = current_line.len() - 1;
                    self.contents[start.y].drain(start_index..end_index);
                }
            }
            else {
                // Did not remove newline, so don't concatenate with following line and simply
                // drain
                let (start_index, _) = *(char_indices.iter().nth(start.x).unwrap());
                let (end_index, _) = *(char_indices.iter().nth(end.x).unwrap());
                self.contents[start.y].drain(start_index..end_index);
            }
        }
        else {
            // If multiple lines are deleted, our strategy is to drain the first line from the
            // start index to the end of the line, drain the last line from the start of the line
            // to one before the end index, and remove all lines in between.

            let mut first_line = self.contents[start.y].clone();
            let first_line_clone = first_line.clone();
            let first_line_grapheme_indices: Vec<(usize, &str)> =
                UniSeg::grapheme_indices(&first_line_clone[..], true).collect();
            let (first_line_start_index, _) =
                *(first_line_grapheme_indices.iter().nth(start.x).unwrap());
            first_line.drain(first_line_start_index..first_line_clone.len());
            let new_first_line = first_line.clone();
            println!("{}", new_first_line);

            let mut last_line = self.contents[end.y].clone();
            let last_line_clone = last_line.clone();
            let last_line_grapheme_indices: Vec<(usize, &str)> =
                UniSeg::grapheme_indices(&last_line_clone[..], true).collect();
            let last_line_end_index: usize;
            if end.x == last_line.len() {
                last_line_end_index = last_line.len()
            }
            else {
                match *(last_line_grapheme_indices.iter().nth(end.x).unwrap()) {
                    (i, _) => last_line_end_index = i
                }
            }
            last_line.drain(0..last_line_end_index);
            let new_last_line = last_line.clone();
            println!("{}", new_last_line);

            for i in (start.y..end.y + 1).rev() {
                self.contents.remove(i);
            }

            if !(new_first_line.is_empty() && new_last_line.is_empty()) {
                self.contents.insert(start.y, new_first_line + &new_last_line);
            }
            else if new_last_line.is_empty() {
                self.contents.insert(start.y, new_first_line + "\n");
            }

            if self.contents.is_empty() {
                self.contents.push("\n".to_string());
            }
        }
    }

    pub fn point(&self) -> Coord {
        self.point.clone()
    }
}
