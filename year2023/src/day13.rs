use super::*;
use crate::utils::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Node {
    Ash,
    Rock,
}

fn col_reflection(mirror: &Grid<Node>, col: isize) -> u64 {
    let mut smudges = 0;
    for (a, b) in (0..col).rev().zip(col..mirror.width()) {
        for y in 0..mirror.height() {
            if mirror[(a, y)] != mirror[(b, y)] {
                smudges += 1;
            }
        }
    }

    smudges
}

fn row_reflection(mirror: &Grid<Node>, row: isize) -> u64 {
    let mut smudges = 0;
    for (a, b) in (0..row).rev().zip(row..mirror.height()) {
        for x in 0..mirror.width() {
            if mirror[(x, a)] != mirror[(x, b)] {
                smudges += 1;
            }
        }
    }

    smudges
}

fn reflection_score(mirror: &Grid<Node>, smudges: u64) -> u64 {
    let mut sum = 0;
    for c in 1..mirror.width() {
        if col_reflection(mirror, c) == smudges {
            sum += c as u64;
        }
    }
    for r in 1..mirror.height() {
        if row_reflection(mirror, r) == smudges {
            sum += 100 * r as u64;
        }
    }

    sum
}

pub struct Day13;
impl Solution for Day13 {
    type Input<'a> = Vec<Grid<Node>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let sum = input
            .iter()
            .map(|mirror| reflection_score(mirror, 0))
            .sum::<u64>();

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let sum = input
            .iter()
            .map(|mirror| reflection_score(mirror, 1))
            .sum::<u64>();

        format!("{}", sum)
    }
}

mod parsing {
    use super::Node;
    use crate::utils::Grid;
    use nom::{
        character::complete::{char, line_ending},
        multi::{many1, separated_list1},
        sequence::tuple,
        *,
    };

    fn mirror(input: &str) -> IResult<&str, Grid<Node>> {
        separated_list1(
            line_ending,
            many1(
                char('#')
                    .map(|_| Node::Rock)
                    .or(char('.').map(|_| Node::Ash)),
            ),
        )
        .map(|m| m.into())
        .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Grid<Node>>> {
        separated_list1(tuple((line_ending, line_ending)), mirror)(input)
    }
}

gen_test!(
    a,
    Day13,
    r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    "405"
);
gen_test!(
    b,
    Day13,
    r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    "400"
);
