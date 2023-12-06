use crate::aoc::Solver;
use itertools::Itertools;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut split1: Vec<&str> = input.split("\n\n").collect();

        let split2: Vec<&str> = split1.remove(0).split(": ").collect();
        let seeds: Vec<&str> = split2[1].split(' ').collect();
        let seeds: Vec<u64> = seeds.iter().map(|s| s.parse().unwrap()).collect();

        let mut maps = Vec::new();

        for map_text in split1 {
            let mut lines: Vec<&str> = map_text.lines().collect();
            let categories = lines.remove(0);

            let split3: Vec<&str> = categories.split(' ').collect();
            let split4: Vec<&str> = split3[0].split('-').collect();

            let mut map = Map {
                source_category: split4[0].to_string(),
                destination_category: split4[2].to_string(),
                ranges: Vec::new(),
            };

            for line in lines {
                let split5: Vec<&str> = line.split(' ').collect();
                map.ranges.push(Range {
                    source_range_start: split5[1].parse().unwrap(),
                    destination_range_start: split5[0].parse().unwrap(),
                    range_length: split5[2].parse().unwrap(),
                })
            }

            maps.push(map);
        }

        Almanac { seeds, maps }
    }
}

#[derive(Debug)]
struct Map {
    source_category: String,
    destination_category: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, number: u64) -> u64 {
        for range in &self.ranges {
            if number >= range.source_range_start
                && number < range.source_range_start + range.range_length
            {
                let diff = number - range.source_range_start;
                return range.destination_range_start + diff;
            }
        }

        number
    }

    fn map_range(&self, start: u64, range_length: u64) -> Vec<(u64, u64)> {
        let mut unmapped = Vec::from([(start, range_length)]);
        let mut mapped = Vec::new();

        while !unmapped.is_empty() {
            let (start, range_length) = unmapped.remove(0);

            let end = start + range_length - 1;
            let mut in_a_range = false;

            for range in &self.ranges {
                let source_range_end = range.source_range_start + range.range_length - 1;

                if start < range.source_range_start {
                    // Starts before the start
                    if end < range.source_range_start {
                        // Starts before the start and ends before the start
                    } else {
                        // Starts before the start and ends after the start
                        if end < source_range_end {
                            // Starts before the start and ends before the end
                            assert_eq!(
                                end - range.source_range_start + 1 + range.source_range_start
                                    - start,
                                range_length
                            );
                            mapped.push((
                                range.destination_range_start,
                                end - range.source_range_start + 1,
                            ));
                            unmapped.push((start, range.source_range_start - start));
                            in_a_range = true;
                            break;
                        } else {
                            // Starts before the start and ends after the end
                            assert_eq!(
                                range.range_length + range.source_range_start - start + end
                                    - source_range_end,
                                range_length
                            );
                            mapped.push((range.destination_range_start, range.range_length));
                            unmapped.push((start, range.source_range_start - start));
                            unmapped.push((source_range_end, end - source_range_end));
                            in_a_range = true;
                            break;
                        }
                    }
                } else {
                    // Starts after the start
                    if start < source_range_end {
                        // Starts before the end
                        if end < source_range_end {
                            // Starts after the start and ends before the end
                            let diff = start - range.source_range_start;
                            mapped.push((range.destination_range_start + diff, range_length));
                            in_a_range = true;
                            break;
                        } else {
                            // Starts after the start and ends after the end
                            let diff = start - range.source_range_start;
                            assert_eq!(
                                source_range_end - start + end - source_range_end + 1,
                                range_length
                            );
                            mapped.push((
                                range.destination_range_start + diff,
                                source_range_end - start,
                            ));
                            unmapped.push((source_range_end, end - source_range_end + 1));
                            in_a_range = true;
                            break;
                        }
                    } else {
                        // Starts after the end
                    }
                }
            }

            if !in_a_range {
                mapped.push((start, range_length));
            }
        }

        mapped
    }
}

#[derive(Debug)]
struct Range {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Day5 {}
    }
}

impl Solver for Day5 {
    fn day(&self) -> i32 {
        5
    }

    fn solve_part_1(&self, input: &str) -> String {
        let almanac = Almanac::parse(input);
        let mut min = std::u64::MAX;

        for seed in &almanac.seeds {
            let mut curr_num = *seed;
            let mut curr_category = "seed";

            while curr_category != "location" {
                let map = almanac
                    .maps
                    .iter()
                    .find(|m| m.source_category == curr_category)
                    .unwrap();

                curr_category = &map.destination_category;
                curr_num = map.map(curr_num);
            }

            if curr_num < min {
                min = curr_num;
            }
        }

        min.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let almanac = Almanac::parse(input);
        let mut min = std::u64::MAX;

        for (start, range_length) in almanac.seeds.iter().tuples() {
            let mut curr_category = "seed";
            let mut ranges = Vec::from([(*start, *range_length)]);

            while curr_category != "location" {
                let map = almanac
                    .maps
                    .iter()
                    .find(|m| m.source_category == curr_category)
                    .unwrap();

                curr_category = &map.destination_category;

                let mut new_ranges: Vec<(u64, u64)> = Vec::new();

                for (start, range_length) in ranges {
                    let mut mapped = map.map_range(start, range_length);
                    new_ranges.append(&mut mapped);
                }

                ranges = new_ranges;
            }
            for (start, _) in ranges {
                if start < min {
                    min = start;
                }
            }
        }

        min.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::aoc::Solver;

    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        let solver = Day5::new();
        assert_eq!(solver.solve_part_1(INPUT), "35");
    }

    #[test]
    fn part_2() {
        let solver = Day5::new();
        assert_eq!(solver.solve_part_2(INPUT), "46");
    }
}
