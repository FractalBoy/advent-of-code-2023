use std::collections::HashMap;

use crate::aoc::Solver;

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Day3 {}
    }

    fn parse_engine(&self, input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    fn check_for_parts(
        &self,
        engine: &Vec<Vec<char>>,
        start_idx: Option<usize>,
        end_idx: Option<usize>,
        line_idx: usize,
        char_pred: &dyn Fn(char) -> bool,
    ) -> Option<(i32, Vec<(usize, usize)>)> {
        if let (Some(start_idx), Some(end_idx)) = (start_idx, end_idx) {
            let line = &engine[line_idx];

            let number: i32 = String::from_iter(&line[start_idx..=end_idx])
                .parse()
                .unwrap();

            let mut coords = Vec::new();

            for idx in self.check_line(&line, start_idx, end_idx, char_pred) {
                coords.push((line_idx, idx));
            }

            if line_idx > 0 {
                let prev_line = &engine[line_idx - 1];

                for idx in self.check_line(&prev_line, start_idx, end_idx, char_pred) {
                    coords.push((line_idx - 1, idx))
                }
            }

            if line_idx < engine.len() - 1 {
                let next_line = &engine[line_idx + 1];

                for idx in self.check_line(&next_line, start_idx, end_idx, char_pred) {
                    coords.push((line_idx + 1, idx));
                }
            }

            if coords.is_empty() {
                return None;
            } else {
                return Some((number, coords));
            }
        }

        None
    }

    fn check_line(
        &self,
        line: &[char],
        start_idx: usize,
        end_idx: usize,
        char_pred: &dyn Fn(char) -> bool,
    ) -> Vec<usize> {
        let start_idx = if start_idx > 0 {
            start_idx - 1
        } else {
            start_idx
        };

        let end_idx = if end_idx < line.len() - 1 {
            end_idx + 1
        } else {
            end_idx
        };

        let mut idxs = Vec::new();

        for idx in start_idx..=end_idx {
            let char = line[idx];

            if char_pred(char) {
                idxs.push(idx);
            }
        }

        idxs
    }
}

impl Solver for Day3 {
    fn day(&self) -> i32 {
        3
    }

    fn solve_part_1(&self, input: &str) -> String {
        let engine = self.parse_engine(input);
        let mut sum = 0;

        for i in 0..engine.len() {
            let line = &engine[i];

            let mut start: Option<usize> = None;
            let mut end: Option<usize> = None;
            let char_pred = |char: char| !char.is_numeric() && char != '.';

            for j in 0..line.len() {
                let char = line[j];

                if char.is_numeric() {
                    if let None = start {
                        start = Some(j);
                    }

                    end = Some(j);
                } else {
                    if let Some((number, _)) =
                        self.check_for_parts(&engine, start, end, i, &char_pred)
                    {
                        sum += number;
                    }

                    start = None;
                    end = None;
                }
            }

            if let Some((number, _)) = self.check_for_parts(&engine, start, end, i, &char_pred) {
                sum += number;
            }
        }

        sum.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let engine = self.parse_engine(input);
        let mut coord_numbers: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        for i in 0..engine.len() {
            let line = &engine[i];

            let mut start: Option<usize> = None;
            let mut end: Option<usize> = None;
            let char_pred = |char: char| char == '*';

            for j in 0..line.len() {
                let char = line[j];

                if char.is_numeric() {
                    if let None = start {
                        start = Some(j);
                    }

                    end = Some(j);
                } else {
                    if let Some((number, coords)) =
                        self.check_for_parts(&engine, start, end, i, &char_pred)
                    {
                        for coord in coords {
                            if let Some(point) = coord_numbers.get_mut(&coord) {
                                point.push(number);
                            } else {
                                coord_numbers.insert(coord, Vec::from([number]));
                            }
                        }
                    }

                    start = None;
                    end = None;
                }
            }

            if let Some((number, coords)) = self.check_for_parts(&engine, start, end, i, &char_pred)
            {
                for coord in coords {
                    if let Some(point) = coord_numbers.get_mut(&coord) {
                        point.push(number);
                    } else {
                        coord_numbers.insert(coord, Vec::from([number]));
                    }
                }
            }
        }

        let mut sum = 0;

        for (_, start_end_numbers) in coord_numbers {
            if start_end_numbers.len() == 2 {
                sum += start_end_numbers[0] * start_end_numbers[1];
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Solver;
    use crate::day3::Day3;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1() {
        let solver = Day3::new();
        assert_eq!(solver.solve_part_1(INPUT), "4361");
    }

    #[test]
    fn part_2() {
        let solver = Day3::new();
        assert_eq!(solver.solve_part_2(INPUT), "467835")
    }
}
