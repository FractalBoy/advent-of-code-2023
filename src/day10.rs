use std::fmt::{Display, Write};

use crate::aoc::Solver;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    StartingPosition,
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Ground,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char: char = (*self).into();
        f.write_char(char)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::StartingPosition,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            'F' => Self::SouthToEast,
            '7' => Self::SouthToWest,
            '.' => Self::Ground,
            value => unreachable!("{} is not a valid tile character", value.escape_debug()),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::StartingPosition => 'S',
            Self::Ground => '.',
            Self::Vertical => '\u{2502}',
            Self::Horizontal => '\u{2500}',
            Self::NorthToWest => '\u{2518}',
            Self::NorthToEast => '\u{2514}',
            Self::SouthToEast => '\u{250C}',
            Self::SouthToWest => '\u{2510}',
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    map: Vec<Tile>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut map = Vec::new();
        let mut width = 0;

        for line in value.lines() {
            width = 0;

            for char in line.chars() {
                width += 1;
                map.push(char.into());
            }
        }

        Self { map, width }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;

        for tile in &self.map {
            if i % self.width == 0 {
                f.write_char('\n')?;
            }

            f.write_char((*tile).into())?;
            i += 1;
        }

        Ok(())
    }
}

impl Map {
    fn get_starting_position(&self) -> usize {
        self.map
            .iter()
            .position(|t| *t == Tile::StartingPosition)
            .unwrap()
    }

    fn get_next_positions(&self, pos: usize) -> (usize, usize) {
        match self.map[pos] {
            Tile::StartingPosition => {
                let mut next_positions = Vec::new();

                if pos % self.width > 0 {
                    let left = pos - 1;

                    match self.map[left] {
                        Tile::Horizontal | Tile::NorthToEast | Tile::SouthToEast => {
                            next_positions.push(left)
                        }
                        _ => (),
                    }
                }

                if pos % self.width < self.width - 1 {
                    let right = pos + 1;

                    match self.map[right] {
                        Tile::Horizontal | Tile::NorthToWest | Tile::SouthToWest => {
                            next_positions.push(right)
                        }
                        _ => (),
                    }
                }

                if pos > self.width {
                    let up = pos - self.width;

                    match self.map[up] {
                        Tile::Vertical | Tile::SouthToEast | Tile::SouthToWest => {
                            next_positions.push(up)
                        }
                        _ => (),
                    }
                }

                if pos + self.width < self.map.len() {
                    let down = pos + self.width;

                    match self.map[down] {
                        Tile::Vertical | Tile::NorthToEast | Tile::NorthToWest => {
                            next_positions.push(down)
                        }
                        _ => (),
                    }
                }

                (next_positions[0], next_positions[1])
            }
            Tile::Vertical => {
                let up = pos - self.width;
                let down = pos + self.width;
                (up, down)
            }
            Tile::Horizontal => {
                let left = pos - 1;
                let right = pos + 1;
                (left, right)
            }
            Tile::NorthToEast => {
                let up = pos - self.width;
                let right = pos + 1;
                (up, right)
            }
            Tile::NorthToWest => {
                let up = pos - self.width;
                let left = pos - 1;
                (up, left)
            }
            Tile::SouthToEast => {
                let down = pos + self.width;
                let right = pos + 1;
                (down, right)
            }
            Tile::SouthToWest => {
                let down = pos + self.width;
                let left = pos - 1;
                (down, left)
            }
            _ => unreachable!(),
        }
    }

    fn walk(&self) -> u32 {
        let start = self.get_starting_position();

        let mut prev_pos1 = start;
        let mut prev_pos2 = start;

        let mut pos1;
        let mut pos2;

        (pos1, pos2) = self.get_next_positions(start);

        let mut count = 1;

        loop {
            if pos1 == pos2 {
                break;
            }

            count += 1;

            let (next_pos1_1, next_pos1_2) = self.get_next_positions(pos1);
            let (next_pos2_1, next_pos2_2) = self.get_next_positions(pos2);

            let next_pos1;
            let next_pos2;

            if next_pos1_1 == prev_pos1 {
                next_pos1 = next_pos1_2;
            } else {
                next_pos1 = next_pos1_1;
            }

            if next_pos2_1 == prev_pos2 {
                next_pos2 = next_pos2_2;
            } else {
                next_pos2 = next_pos2_1;
            }

            prev_pos1 = pos1;
            prev_pos2 = pos2;

            pos1 = next_pos1;
            pos2 = next_pos2;
        }

        count
    }
}

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Day10 {}
    }
}

impl Solver for Day10 {
    fn day(&self) -> i32 {
        10
    }

    fn solve_part_1(&self, input: &str) -> String {
        let map: Map = input.into();
        map.walk().to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use crate::aoc::Solver;

    #[test]
    fn part_1() {
        let solver = Day10::new();

        assert_eq!(
            solver.solve_part_1(
                ".....
.S-7.
.|.|.
.L-J.
.....",
            ),
            "4"
        );
        assert_eq!(
            solver.solve_part_1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
            ),
            "8"
        );
    }

    #[test]
    fn part_2() {
        let solver = Day10::new();
        assert_eq!(
            solver.solve_part_2(
                "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ..........."
            ),
            "4"
        );
        assert_eq!(
            solver.solve_part_2(
                ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ..."
            ),
            "8"
        );
        assert_eq!(
            solver.solve_part_2(
                "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L"
            ),
            "10"
        );
    }
}
