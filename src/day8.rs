use std::collections::HashMap;

use crate::aoc::Solver;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct InstructionList(Vec<Instruction>);

impl From<&str> for InstructionList {
    fn from(value: &str) -> Self {
        InstructionList(value.chars().map(|ch| ch.into()).collect())
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(' ').collect();
        let label = split[0];
        let left = split[2].replace('(', "").replace(',', "");
        let right = split[3].replace(')', "");

        Node {
            label: label.to_owned(),
            left,
            right,
        }
    }
}

#[derive(Debug)]
struct NodeList(HashMap<String, Node>);

impl From<&str> for NodeList {
    fn from(value: &str) -> Self {
        NodeList(HashMap::from_iter(
            value
                .lines()
                .map(|l| l.into())
                .map(|n: Node| (n.label.clone(), n)),
        ))
    }
}

impl NodeList {
    fn navigate(&self, node: &Node, instruction: &Instruction) -> &Node {
        match instruction {
            Instruction::Left => self.0.get(&node.left).unwrap(),
            Instruction::Right => self.0.get(&node.right).unwrap(),
        }
    }
}

#[derive(Debug)]
struct InstructionAndNodeList(InstructionList, NodeList);

impl From<&str> for InstructionAndNodeList {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split("\n\n").collect();
        InstructionAndNodeList(split[0].into(), split[1].into())
    }
}

pub struct Day8 {}

impl Day8 {
    pub fn new() -> Self {
        Day8 {}
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solver for Day8 {
    fn day(&self) -> i32 {
        8
    }

    fn solve_part_1(&self, input: &str) -> String {
        let instruction_and_node_list: InstructionAndNodeList = input.into();
        let instruction_list = &instruction_and_node_list.0;
        let node_list = &instruction_and_node_list.1;
        let mut node = node_list.0.get("AAA").unwrap();

        let mut count = 0;

        for instruction in instruction_list.0.clone().into_iter().cycle() {
            node = node_list.navigate(node, &instruction);
            count += 1;

            if node.label == "ZZZ" {
                break;
            }
        }

        count.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let instruction_and_node_list: InstructionAndNodeList = input.into();
        let instruction_list = &instruction_and_node_list.0;
        let node_list = &instruction_and_node_list.1;
        let mut nodes: Vec<&Node> = Vec::new();

        for (label, node) in &node_list.0 {
            if label.ends_with("A") {
                nodes.push(node);
            }
        }

        let mut lcm = 1;

        for node in &mut nodes {
            let mut count = 0;

            for instruction in instruction_list.0.clone().into_iter().cycle() {
                *node = node_list.navigate(node, &instruction);
                count += 1;

                if node.label.ends_with("Z") {
                    break;
                }
            }

            lcm = lcm * count / gcd(lcm, count);
        }

        lcm.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::aoc::Solver;

    #[test]
    fn part_1() {
        let solver = Day8::new();
        assert_eq!(
            solver.solve_part_1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2"
        );
        assert_eq!(
            solver.solve_part_1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "6"
        );
    }

    #[test]
    fn part_2() {
        let solver = Day8::new();
        assert_eq!(
            solver.solve_part_2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            "6"
        );
    }
}
