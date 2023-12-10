use crate::aoc::Solver;

#[derive(Debug)]
struct History(Vec<i32>);

impl From<&str> for History {
    fn from(value: &str) -> Self {
        History(value.split(' ').map(|n| n.parse().unwrap()).collect())
    }
}

impl History {
    fn diffs(&self) -> Self {
        History(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    fn all_zero(&self) -> bool {
        self.0.iter().all(|n| *n == 0)
    }

    fn get_next_value(&self) -> i32 {
        if self.all_zero() {
            return 0;
        }

        self.0.last().unwrap() + self.diffs().get_next_value()
    }

    fn get_previous_value(&self) -> i32 {
        if self.all_zero() {
            return 0;
        }

        self.0[0] - self.diffs().get_previous_value()
    }
}

struct Histories(Vec<History>);

impl From<&str> for Histories {
    fn from(value: &str) -> Self {
        Histories(value.lines().map(|l| l.into()).collect())
    }
}

pub struct Day9 {}

impl Day9 {
    pub fn new() -> Self {
        Day9 {}
    }
}

impl Solver for Day9 {
    fn day(&self) -> i32 {
        9
    }

    fn solve_part_1(&self, input: &str) -> String {
        let histories: Histories = input.into();
        let mut sum = 0;

        for history in histories.0 {
            sum += history.get_next_value();
        }

        sum.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let histories: Histories = input.into();
        let mut sum = 0;

        for history in histories.0 {
            sum += history.get_previous_value();
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day9;
    use crate::aoc::Solver;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1() {
        let solver = Day9::new();
        assert_eq!(solver.solve_part_1(INPUT), "114");
    }

    #[test]
    fn part_2() {
        let solver = Day9::new();
        assert_eq!(solver.solve_part_2(INPUT), "2");
    }
}
