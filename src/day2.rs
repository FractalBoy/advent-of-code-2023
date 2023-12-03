use crate::aoc::Solver;
use once_cell::sync::Lazy;
use regex::Regex;

struct Set {
    green: i32,
    blue: i32,
    red: i32,
}

static SET_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<count>\d+) (?P<color>blue|red|green)(?:, )?").unwrap());

impl Set {
    fn new() -> Self {
        Set {
            green: 0,
            blue: 0,
            red: 0,
        }
    }

    fn set_count(&mut self, color: &str, count: i32) {
        match color {
            "green" => self.green = count,
            "red" => self.red = count,
            "blue" => self.blue = count,
            _ => (),
        }
    }

    fn parse(set_str: &str) -> Self {
        let mut set = Self::new();

        for capture in SET_REGEX.captures_iter(set_str) {
            let count: i32 = capture["count"].parse().unwrap();
            set.set_count(&capture["color"], count)
        }

        set
    }
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?P<game>\d+): ").unwrap());

impl Game {
    fn new(id: i32) -> Self {
        Game {
            id,
            sets: Vec::new(),
        }
    }

    fn add_set(&mut self, set: Set) {
        self.sets.push(set)
    }

    fn parse(line: &str) -> Self {
        let id: i32 = GAME_REGEX.captures(line).unwrap()["game"].parse().unwrap();
        let mut game = Self::new(id);

        let line = GAME_REGEX.replace(line, "");

        for set_str in line.split("; ") {
            game.add_set(Set::parse(set_str));
        }

        game
    }
}

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Self {
        Day2 {}
    }

    fn parse_games(&self, input: &str) -> Vec<Game> {
        let mut games = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            games.push(Game::parse(line))
        }

        games
    }
}

impl Solver for Day2 {
    fn day(&self) -> i32 {
        2
    }

    fn solve_part_1(&self, input: &str) -> String {
        const MAX_RED: i32 = 12;
        const MAX_GREEN: i32 = 13;
        const MAX_BLUE: i32 = 14;

        let games = self.parse_games(input);
        let mut sum = 0;

        for game in games {
            let mut possible = true;

            for set in game.sets {
                if set.red > MAX_RED || set.green > MAX_GREEN || set.blue > MAX_BLUE {
                    possible = false;
                }
            }

            if possible {
                sum += game.id;
            }
        }

        sum.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let games = self.parse_games(input);
        let mut sum = 0;

        for game in games {
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;

            for set in game.sets {
                if set.red > min_red {
                    min_red = set.red
                }
                if set.green > min_green {
                    min_green = set.green
                }
                if set.blue > min_blue {
                    min_blue = set.blue
                }
            }

            let power = min_red * min_green * min_blue;
            sum += power;
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Solver;
    use crate::day2::Day2;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        let solver = Day2::new();
        assert_eq!(solver.solve_part_1(INPUT), "8");
    }

    #[test]
    fn part_2() {
        let solver = Day2::new();
        assert_eq!(solver.solve_part_2(INPUT), "2286")
    }
}
