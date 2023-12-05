use crate::aoc::Solver;

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Day1 {}
    }
}

#[derive(Debug)]
enum State {
    NoLetters,
    OneLetter(char),
    TwoLetters(char, char),
    ThreeLetters(char, char, char),
    FourLetters(char, char, char, char),
    FiveLetters(char, char, char, char, char),
}

impl State {
    fn next_state(self, char: char) -> Self {
        match self {
            Self::NoLetters => match char {
                'o' | 't' | 'f' | 's' | 'e' | 'n' => Self::OneLetter(char),
                _ => Self::NoLetters,
            },
            Self::OneLetter(prev) => match (prev, char) {
                ('o', 'n')
                | ('t', 'w')
                | ('t', 'h')
                | ('f', 'o')
                | ('f', 'i')
                | ('s', 'i')
                | ('s', 'e')
                | ('e', 'i')
                | ('n', 'i') => Self::TwoLetters(prev, char),
                _ => Self::NoLetters.next_state(char),
            },
            Self::TwoLetters(one, two) => match (one, two, char) {
                ('o', 'n', 'e')
                | ('t', 'w', 'o')
                | ('t', 'h', 'r')
                | ('f', 'o', 'u')
                | ('f', 'i', 'v')
                | ('s', 'i', 'x')
                | ('s', 'e', 'v')
                | ('e', 'i', 'g')
                | ('n', 'i', 'n') => Self::ThreeLetters(one, two, char),
                (_, 'n', 'i') | (_, 'o', 'n') | (_, 'e', 'i') => Self::TwoLetters(two, char),
                _ => Self::NoLetters.next_state(char),
            },
            Self::ThreeLetters(one, two, three) => match (one, two, three, char) {
                ('t', 'h', 'r', 'e')
                | ('f', 'o', 'u', 'r')
                | ('f', 'i', 'v', 'e')
                | ('s', 'e', 'v', 'e')
                | ('e', 'i', 'g', 'h')
                | ('n', 'i', 'n', 'e') => Self::FourLetters(one, two, three, char),
                (_, _, 'e', 'i') | (_, _, 'o', 'n') | (_, _, 'n', 'i') => {
                    Self::TwoLetters(three, char)
                }
                _ => Self::NoLetters.next_state(char),
            },
            Self::FourLetters(one, two, three, four) => match (one, two, three, four, char) {
                ('t', 'h', 'r', 'e', 'e')
                | ('s', 'e', 'v', 'e', 'n')
                | ('e', 'i', 'g', 'h', 't') => Self::FiveLetters(one, two, three, four, char),
                (_, _, _, 'e', 'i') => Self::TwoLetters(four, char),
                _ => Self::NoLetters.next_state(char),
            },
            Self::FiveLetters(_, _, _, _, prev) => match (prev, char) {
                ('t', 'w') | ('t', 'h') | ('e', 'i') | ('n', 'i') => Self::TwoLetters(prev, char),
                _ => Self::NoLetters.next_state(char),
            },
        }
    }

    fn get_number(&self) -> Option<char> {
        match &self {
            Self::ThreeLetters('o', 'n', 'e') => Some('1'),
            Self::ThreeLetters('t', 'w', 'o') => Some('2'),
            Self::FiveLetters('t', 'h', 'r', 'e', 'e') => Some('3'),
            Self::FourLetters('f', 'o', 'u', 'r') => Some('4'),
            Self::FourLetters('f', 'i', 'v', 'e') => Some('5'),
            Self::ThreeLetters('s', 'i', 'x') => Some('6'),
            Self::FiveLetters('s', 'e', 'v', 'e', 'n') => Some('7'),
            Self::FiveLetters('e', 'i', 'g', 'h', 't') => Some('8'),
            Self::FourLetters('n', 'i', 'n', 'e') => Some('9'),
            _ => None,
        }
    }
}

impl Solver for Day1 {
    fn day(&self) -> i32 {
        1
    }

    fn solve_part_1(&self, input: &str) -> String {
        let mut sum = 0;

        for line in input.lines() {
            let mut first: Option<char> = Option::None;
            let mut last: Option<char> = Option::None;

            for char in line.chars() {
                if char.is_numeric() {
                    if let None = first {
                        first = Some(char)
                    }
                    if char.is_numeric() {
                        last = Some(char);
                    }
                }
            }

            if let (Some(first), Some(last)) = (first, last) {
                let mut num = String::new();
                num.push(first);
                num.push(last);

                if let Ok(num) = num.parse::<i32>() {
                    sum += num;
                }
            }
        }

        sum.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let mut sum = 0;

        for line in input.split("\n") {
            let mut state = State::NoLetters;
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            for char in line.chars() {
                state = state.next_state(char);

                let num;

                if char.is_numeric() {
                    num = Some(char);
                } else {
                    num = state.get_number()
                }

                if let Some(num) = num {
                    if let None = first {
                        first = Some(num);
                    }

                    last = Some(num);
                }
            }

            if let (Some(first), Some(last)) = (first, last) {
                let mut num = String::new();
                num.push(first);
                num.push(last);

                if let Ok(num) = num.parse::<i32>() {
                    sum += num;
                }
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::aoc::Solver;

    #[test]
    fn part_1() {
        let solver = Day1::new();
        assert_eq!(
            solver.solve_part_1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142"
        );
    }

    #[test]
    fn part_2() {
        let solver = Day1::new();
        assert_eq!(
            solver.solve_part_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281"
        );
    }
}
