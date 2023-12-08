use itertools::Itertools;
use num_integer::{ExtendedGcd, Integer};
use std::collections::HashMap;

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    left: u32,
    right: u32,
}

impl Node {
    fn get(&self, direction: Direction) -> u32 {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    fn index(label: &str) -> u32 {
        let label = label.as_bytes();
        (label[0] as u32) * 256 * 256 + (label[1] as u32) * 256 + label[2] as u32
    }

    fn is_start(node: u32) -> bool {
        node % 256 == 'A' as u32
    }

    fn is_end(node: u32) -> bool {
        node % 256 == 'Z' as u32
    }
}

#[derive(Clone, Debug)]
pub struct Cycle {
    offset: u64,
    length: u64,
    accepted_start: Vec<u64>,
    accepted_cycle: Vec<u64>,
}

impl Cycle {
    fn trace(instructions: &[Direction], graph: &HashMap<u32, Node>, start: u32) -> Self {
        let mut current = start;
        let mut steps = 0;

        let instruction_count = instructions.len() as u64;
        let mut instructions = instructions.iter().cycle().copied();

        let mut accepted = Vec::new();
        let mut visited = HashMap::new();
        visited.insert(start, 0);

        let offset = loop {
            steps += 1;

            current = graph[&current].get(instructions.next().unwrap());
            if Node::is_end(current) {
                accepted.push(steps);
            }

            let offset = *visited.entry(current).or_insert(steps);
            if steps > offset && (steps - offset) % instruction_count == 0 {
                break offset;
            }
        };

        let length = steps - offset;

        let accepted_start_length = accepted.iter().filter(|&&x| x < offset).count();

        let accepted_start = Vec::from(&accepted[..accepted_start_length]);
        let accepted_cycle = Vec::from_iter(
            (&accepted[accepted_start_length..])
                .iter()
                .map(|&x| x - offset),
        );

        Self {
            offset,
            length,
            accepted_start,
            accepted_cycle,
        }
    }

    fn is_accepted(&self, x: u64) -> bool {
        if x < self.offset {
            self.accepted_start.binary_search(&x).is_ok()
        } else {
            let x = (x - self.offset) % self.length;
            self.accepted_cycle.binary_search(&x).is_ok()
        }
    }

    fn first_accepted(&self) -> Option<u64> {
        if let Some(first) = self.accepted_start.first() {
            return Some(*first);
        }

        if let Some(first) = self.accepted_cycle.first() {
            return Some(*first + self.offset);
        }

        None
    }

    fn union(&self, other: &Self) -> Self {
        if other.offset > self.offset {
            return other.union(self);
        }

        let offset = self.offset;
        let (
            ExtendedGcd {
                gcd: d,
                x: c1,
                y: c2,
            },
            length,
        ) = (self.length as i64).extended_gcd_lcm(&(other.length as i64));
        let e1 = (c1 * self.length as i64).rem_euclid(length) as u64;
        let e2 = (c2 * other.length as i64).rem_euclid(length) as u64;
        let length = length as u64;

        let accepted_start = self
            .accepted_start
            .iter()
            .copied()
            .filter(|&x| other.is_accepted(x))
            .collect();

        let accepted_cycle = self
            .accepted_cycle
            .iter()
            .cartesian_product(other.accepted_cycle.iter())
            .filter_map(|(&x1, &x2)| {
                let a1 = (x1 + self.offset).rem_euclid(self.length);
                let a2 = (x2 + other.offset).rem_euclid(other.length);
                if a1 % d as u64 != 0 || a2 % d as u64 != 0 {
                    return None;
                }
                let b1 = a1 / d as u64;
                let b2 = a2 / d as u64;

                let mut x = e2 * b1 + e1 * b2;
                if x < offset {
                    x += length;
                }

                (x - offset).rem_euclid(length).into()
            })
            .collect();

        Self {
            offset,
            length,
            accepted_start,
            accepted_cycle,
        }
    }
}

pub struct Day08;
impl Solution for Day08 {
    type Input<'a> = (Vec<Direction>, HashMap<u32, Node>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut current = Node::index("AAA");
        let target = Node::index("ZZZ");

        let directions = input.0.iter().cycle();

        for (step, direction) in directions.enumerate() {
            if current == target {
                return format!("{}", step);
            }
            current = input.1[&current].get(*direction);
        }

        unreachable!()
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let cycles = input
            .1
            .keys()
            .filter(|&&n| Node::is_start(n))
            .map(|&n| Cycle::trace(&input.0, &input.1, n))
            .collect::<Vec<_>>();

        let union_cycle = cycles
            .into_iter()
            .reduce(|x, y| Cycle::union(&x, &y))
            .unwrap();

        format!("{}", union_cycle.first_accepted().unwrap())
    }
}

mod parsing {
    use std::collections::HashMap;

    use super::Direction;
    use super::Node;
    use nom::{
        bytes::complete::tag,
        character::complete::char,
        character::complete::{alphanumeric1, line_ending},
        multi::{many1, separated_list1},
        sequence::tuple,
        *,
    };

    fn node(input: &str) -> IResult<&str, (u32, Node)> {
        tuple((
            alphanumeric1,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")"),
        ))
        .map(|(x, _, l, _, r, _)| {
            (
                Node::index(x),
                Node {
                    left: Node::index(l),
                    right: Node::index(r),
                },
            )
        })
        .parse(input)
    }

    fn direction(input: &str) -> IResult<&str, Direction> {
        char('R')
            .or(char('L'))
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => unreachable!(),
            })
            .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<u32, Node>)> {
        many1(direction)
            .and(many1(line_ending))
            .and(separated_list1(line_ending, node))
            .map(|((instr, _), nodes)| (instr, HashMap::from_iter(nodes.into_iter())))
            .parse(input)
    }
}

gen_test!(
    a,
    Day08,
    r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    "2"
);
gen_test!(
    b,
    Day08,
    r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    "6"
);
