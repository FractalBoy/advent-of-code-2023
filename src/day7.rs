use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::aoc::Solver;

trait Card:
    Sized + Default + Clone + Copy + core::hash::Hash + PartialEq + Eq + PartialOrd + Ord + From<char>
{
    fn strength(hand: Hand<Self>) -> Strength;
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart1 {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for CardPart1 {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

impl From<char> for CardPart2 {
    fn from(value: char) -> Self {
        match value {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

impl From<CardPart2> for CardPart1 {
    fn from(value: CardPart2) -> Self {
        match value {
            CardPart2::Joker => CardPart1::Two,
            CardPart2::Two => CardPart1::Three,
            CardPart2::Three => CardPart1::Four,
            CardPart2::Four => CardPart1::Five,
            CardPart2::Five => CardPart1::Six,
            CardPart2::Six => CardPart1::Seven,
            CardPart2::Seven => CardPart1::Eight,
            CardPart2::Eight => CardPart1::Nine,
            CardPart2::Nine => CardPart1::Ten,
            CardPart2::Ten => CardPart1::Jack,
            CardPart2::Queen => CardPart1::Queen,
            CardPart2::King => CardPart1::King,
            CardPart2::Ace => CardPart1::Ace,
        }
    }
}

impl From<Hand<CardPart2>> for Hand<CardPart1> {
    fn from(value: Hand<CardPart2>) -> Self {
        Hand(value.0.map(|c| c.into()))
    }
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart2 {
    #[default]
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Hand<T: Card>([T; 5]);

impl<T: Card> Index<usize> for Hand<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T: Card> IndexMut<usize> for Hand<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}

impl<T: Card> From<&str> for Hand<T> {
    fn from(value: &str) -> Self {
        let mut hand = Hand([Default::default(); 5]);

        for (idx, c) in value.char_indices() {
            hand[idx] = c.into()
        }

        hand
    }
}

impl<T: Card> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Card> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_strength: Strength = (*self).into();
        let their_strength: Strength = (*other).into();

        if my_strength != their_strength {
            my_strength.cmp(&their_strength)
        } else {
            for (c1, c2) in self.0.into_iter().zip(other.0) {
                if c1 != c2 {
                    return c1.cmp(&c2);
                }
            }

            Ordering::Equal
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<T: Card> Into<Strength> for Hand<T> {
    fn into(self) -> Strength {
        T::strength(self)
    }
}

impl<T: Card> Hand<T> {
    fn counts(&self) -> HashMap<T, u32> {
        self.0.into_iter().fold(HashMap::new(), |mut acc, c| {
            if let Some(count) = acc.get_mut(&c) {
                *count += 1;
            } else {
                acc.insert(c, 1);
            }

            acc
        })
    }
}

impl Card for CardPart1 {
    fn strength(hand: Hand<Self>) -> Strength {
        let counts = hand.counts();

        let mut five = false;
        let mut four = false;
        let mut three = false;
        let mut pairs = 0;

        for (_, count) in counts {
            match count {
                5 => {
                    five = true;
                    break;
                }
                4 => {
                    four = true;
                    break;
                }
                3 => {
                    three = true;
                }
                2 => pairs += 1,
                _ => (),
            }
        }

        if five {
            Strength::FiveOfAKind
        } else if four {
            Strength::FourOfAKind
        } else if three {
            if pairs != 0 {
                Strength::FullHouse
            } else {
                Strength::ThreeOfAKind
            }
        } else {
            match pairs {
                0 => Strength::HighCard,
                1 => Strength::OnePair,
                2 => Strength::TwoPair,
                _ => unreachable!(),
            }
        }
    }
}

impl Card for CardPart2 {
    fn strength(hand: Hand<Self>) -> Strength {
        let mut counts = hand.counts();

        let jokers = counts.remove(&CardPart2::Joker).unwrap_or(0);

        let mut four = false;
        let mut three = false;
        let mut pairs = 0;

        for (_, count) in counts {
            match count {
                4 => {
                    four = true;
                    break;
                }
                3 => three = true,
                2 => pairs += 1,
                _ => (),
            }
        }

        match jokers {
            5 | 4 => Strength::FiveOfAKind,
            3 => {
                if pairs != 0 {
                    Strength::FiveOfAKind
                } else {
                    Strength::FourOfAKind
                }
            }
            2 => {
                if three {
                    Strength::FiveOfAKind
                } else if pairs != 0 {
                    Strength::FourOfAKind
                } else {
                    Strength::ThreeOfAKind
                }
            }
            1 => {
                if four {
                    Strength::FiveOfAKind
                } else if three {
                    Strength::FourOfAKind
                } else {
                    match pairs {
                        2 => Strength::FullHouse,
                        1 => Strength::ThreeOfAKind,
                        0 => Strength::OnePair,
                        _ => unreachable!(),
                    }
                }
            }
            0 => CardPart1::strength(hand.into()),
            _ => unreachable!(),
        }
    }
}

struct HandWithBid<T: Card>(Hand<T>, u32);

impl<T: Card> From<&str> for HandWithBid<T> {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(' ').collect();

        let hand: Hand<T> = split[0].into();
        let bid: u32 = split[1].parse().unwrap();

        HandWithBid(hand, bid)
    }
}

pub struct Day7 {}

impl Day7 {
    pub fn new() -> Self {
        Day7 {}
    }

    fn solve<T: Card>(&self, input: &str) -> String {
        let mut hands_with_bids: Vec<HandWithBid<T>> = input.lines().map(|l| l.into()).collect();
        hands_with_bids.sort_unstable_by(|h1, h2| h1.0.cmp(&h2.0));

        let mut rank = 1;
        let mut score = 0;

        for hand in hands_with_bids {
            score += hand.1 * rank;
            rank += 1;
        }

        score.to_string()
    }
}

impl Solver for Day7 {
    fn day(&self) -> i32 {
        7
    }

    fn solve_part_1(&self, input: &str) -> String {
        self.solve::<CardPart1>(input)
    }

    fn solve_part_2(&self, input: &str) -> String {
        self.solve::<CardPart2>(input)
    }
}

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::aoc::Solver;

    const INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        let solver = Day7::new();
        assert_eq!(solver.solve_part_1(INPUT), "6440");
    }

    #[test]
    fn part_2() {
        let solver = Day7::new();
        assert_eq!(solver.solve_part_2(INPUT), "5905");
    }
}
