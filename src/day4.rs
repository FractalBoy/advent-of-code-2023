use std::collections::HashMap;

use crate::aoc::Solver;

struct Card {
    number: u32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let split1: Vec<&str> = line.split(':').collect();
        let number: u32 = split1[0]
            .replace("Card", "")
            .replace(' ', "")
            .parse()
            .unwrap();

        let split2: Vec<&str> = split1[1].split(" | ").collect();

        let mut winning_numbers: Vec<i32> = split2[0]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        winning_numbers.sort();

        let mut numbers: Vec<i32> = split2[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        numbers.sort();

        Card {
            number,
            winning_numbers,
            numbers,
        }
    }

    fn winning_count(&self) -> u32 {
        let mut count = 0;

        for winning_number in &self.winning_numbers {
            for number in &self.numbers {
                if number > winning_number {
                    break;
                }

                if number == winning_number {
                    count += 1;
                }
            }
        }

        count
    }
}

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }
}

impl Solver for Day4 {
    fn day(&self) -> i32 {
        4
    }

    fn solve_part_1(&self, input: &str) -> String {
        let mut score = 0;

        for line in input.lines() {
            let card = Card::parse(line);
            let count = card.winning_count();

            if count != 0 {
                score += 2i32.pow(count - 1);
            }
        }

        score.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let mut cards: Vec<Card> = input.lines().map(|l| Card::parse(l)).collect();
        cards.sort_by(|a, b| a.number.cmp(&b.number));
        let cards = cards;

        let mut card_counts: HashMap<u32, u32> =
            HashMap::from_iter(cards.iter().map(|c| (c.number, 1)));

        for card in cards {
            let count = card.winning_count();
            let copies = *card_counts.get(&card.number).unwrap();

            for idx in (card.number + 1)..=(card.number + count) {
                if let Some(count) = card_counts.get_mut(&idx) {
                    *count += copies;
                } else {
                    card_counts.insert(idx, copies);
                }
            }
        }

        let sum: u32 = card_counts.values().sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::aoc::Solver;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn part_1() {
        let solver = Day4::new();
        assert_eq!(solver.solve_part_1(INPUT), "13");
    }

    #[test]
    fn part_2() {
        let solver = Day4::new();
        assert_eq!(solver.solve_part_2(INPUT), "30");
    }
}
