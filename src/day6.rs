use crate::aoc::Solver;

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

type Races = Vec<Race>;

impl Race {
    fn parse_races(input: &str) -> Races {
        let lines: Vec<&str> = input.lines().collect();

        lines[0]
            .split(' ')
            .skip(1)
            .filter(|t| !t.is_empty())
            .map(|t| t.trim().parse().unwrap())
            .zip(
                lines[1]
                    .split(' ')
                    .skip(1)
                    .filter(|t| !t.is_empty())
                    .map(|d| d.trim().parse().unwrap()),
            )
            .map(|(t, d)| Race {
                time: t,
                record_distance: d,
            })
            .collect()
    }
}

pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Day6 {}
    }

    fn solve(&self, races: Races) -> u64 {
        let mut ways = Vec::new();

        for race in races {
            let mut min = 0;
            let mut max = race.time / 2 - 1;

            loop {
                if min == max {
                    break;
                }

                let midpoint = (min + max) / 2;
                let distance = (race.time - midpoint) * midpoint;

                if distance > race.record_distance {
                    max -= 1;
                } else if distance <= race.record_distance {
                    min += 1;
                }
            }

            if race.time % 2 == 0 {
                let max = race.time / 2 + (race.time / 2 - min);
                ways.push(max - min + 1);
            } else {
                let max = race.time / 2 + 1 + (race.time / 2 - min);
                ways.push(max - min + 1);
            }
        }

        ways.into_iter().product::<u64>()
    }
}

impl Solver for Day6 {
    fn day(&self) -> i32 {
        6
    }

    fn solve_part_1(&self, input: &str) -> String {
        let races = Race::parse_races(input);
        self.solve(races).to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let races = Race::parse_races(input);

        let mut time = String::new();
        let mut distance = String::new();

        for race in races {
            time.push_str(&race.time.to_string());
            distance.push_str(&race.record_distance.to_string());
        }

        let race = Race {
            time: time.parse().unwrap(),
            record_distance: distance.parse().unwrap(),
        };

        self.solve(Vec::from([race])).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::aoc::Solver;

    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        let solver = Day6::new();
        assert_eq!(solver.solve_part_1(INPUT), "288");
    }

    #[test]
    fn part_2() {
        let solver = Day6::new();
        assert_eq!(solver.solve_part_2(INPUT), "71503");
    }
}
