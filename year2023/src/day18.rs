use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    R,
    L,
    D,
    U,
}

impl Direction {
    fn as_vec(&self) -> (i64, i64) {
        match self {
            Direction::R => (1, 0),
            Direction::L => (-1, 0),
            Direction::U => (0, -1),
            Direction::D => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    direction: Direction,
    steps: u32,
    color: u32,
}

fn calc_area(instructions: impl IntoIterator<Item = (Direction, i64)>) -> i64 {
    let mut position = (0, 0);
    let (int, len) = instructions
        .into_iter()
        .map(|(d, s)| {
            let d = d.as_vec();
            position = (position.0 + s * d.0, position.1 + s * d.1);

            (s * (position.1 * d.0 - position.0 * d.1), s)
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    (int.abs() + len) / 2 + 1
}

pub struct Day18;
impl Solution for Day18 {
    type Input<'a> = Vec<Instruction>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let area = calc_area(input.iter().map(|i| (i.direction, i.steps as i64)));

        format!("{}", area)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let area = calc_area(input.iter().map(|i| {
            let direction = match i.color % 4 {
                0 => Direction::R,
                1 => Direction::D,
                2 => Direction::L,
                3 => Direction::U,
                _ => unreachable!(),
            };
            let steps = (i.color >> 4) as i64;
            (direction, steps)
        }));

        format!("{}", area)
    }
}

mod parsing {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, hex_digit1, line_ending, space1, u32},
        multi::separated_list1,
        sequence::{delimited, tuple},
        *,
    };

    use super::{Direction, Instruction};

    fn direction(input: &str) -> IResult<&str, Direction> {
        char('R')
            .map(|_| Direction::R)
            .or(char('L').map(|_| Direction::L))
            .or(char('U').map(|_| Direction::U))
            .or(char('D').map(|_| Direction::D))
            .parse(input)
    }

    fn color(input: &str) -> IResult<&str, u32> {
        delimited(tag("(#"), hex_digit1, tag(")"))
            .map(|h| u32::from_str_radix(h, 16).unwrap())
            .parse(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        tuple((direction, space1, u32, space1, color))
            .map(|(direction, _, steps, _, color)| Instruction {
                direction,
                steps,
                color,
            })
            .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(line_ending, instruction)(input)
    }
}

gen_test!(
    a,
    Day18,
    r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    "62"
);
gen_test!(
    b,
    Day18,
    r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    "952408144115"
);
