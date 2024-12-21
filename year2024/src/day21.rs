use std::collections::HashMap;

use itertools::Itertools;

use super::*;

pub struct Day21;
impl Solution for Day21 {
    type Input<'a> = Vec<String>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content.lines().map(str::to_owned).collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        format!("{}", solve(&input, 2))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        format!("{}", solve(&input, 25))
    }
}

fn solve(input: &[String], steps_between: u32) -> u64 {
    let mut mem = HashMap::new();
    input
        .iter()
        .map(|seq| {
            let presses: u64 = numbers_to_movement(seq)
                .map(|(x, restricted)| length(x, false, restricted, steps_between, &mut mem) + 1)
                .sum();
            let value: u64 = seq[..(seq.len() - 1)].parse().unwrap();
            presses * value
        })
        .sum()
}

fn length(
    dir: (i32, i32),
    restricted_top: bool,
    restricted_bottom: bool,
    level: u32,
    mem: &mut HashMap<((i32, i32), bool, bool, u32), u64>,
) -> u64 {
    if let Some(v) = mem.get(&(dir, restricted_top, restricted_bottom, level)) {
        return *v;
    }
    let movement = if level == 0 {
        0
    } else {
        let mut l = |x, y, r| length((x, y), r, false, level - 1, mem);
        match (dir.0.signum(), dir.1.signum()) {
            (-1, 0) => l(-2, 1, true) + l(2, -1, true),
            (1, 0) => l(0, 1, false) + l(0, -1, false),
            (0, -1) => l(-1, 0, false) + l(1, 0, false),
            (0, 1) => l(-1, 1, false) + l(1, -1, false),
            (1, 1) => u64::min(
                l(0, 1, false) + l(-1, 0, false) + l(1, -1, false),
                if !restricted_bottom {
                    l(-1, 1, false) + l(1, 0, false) + l(0, -1, false)
                } else {
                    u64::MAX
                },
            ),
            (1, -1) => u64::min(
                l(0, 1, false) + l(-1, -1, false) + l(1, 0, false),
                if !restricted_top {
                    l(-1, 0, false) + l(1, 1, false) + l(0, -1, false)
                } else {
                    u64::MAX
                },
            ),
            (-1, 1) => u64::min(
                l(-1, 1, false) + l(-1, 0, false) + l(2, -1, true),
                if !restricted_top {
                    l(-2, 1, true) + l(1, 0, false) + l(1, -1, false)
                } else {
                    u64::MAX
                },
            ),
            (-1, -1) => u64::min(
                l(-1, 0, false) + l(-1, 1, true) + l(2, -1, true),
                if !restricted_bottom {
                    l(-2, 1, true) + l(1, -1, true) + l(1, 0, false)
                } else {
                    u64::MAX
                },
            ),
            (_, _) => unreachable!(),
        }
    };

    let res = movement + (dir.0.abs() + dir.1.abs()) as u64;
    mem.insert((dir, restricted_top, restricted_bottom, level), res);
    res
}

static NUM_PAD: phf::Map<char, (i32, i32)> = phf::phf_map! {
    '9' => (2,0),
    '8' => (1,0),
    '7' => (0,0),
    '6' => (2,1),
    '5' => (1,1),
    '4' => (0,1),
    '3' => (2,2),
    '2' => (1,2),
    '1' => (0,2),
    '0' => (1,3),
    'A' => (2,3),
};

fn numbers_to_movement<'a>(seq: &'a str) -> impl Iterator<Item = ((i32, i32), bool)> + 'a {
    std::iter::once('A')
        .chain(seq.chars())
        .map(|c| NUM_PAD.get(&c).unwrap())
        .tuple_windows()
        .map(|(a, b)| {
            (
                ((b.0 - a.0), (b.1 - a.1)),
                (a.1 == 3 && b.0 == 0) || (b.1 == 3 && a.0 == 0),
            )
        })
}

gen_test!(
    a,
    Day21,
    r"029A
980A
179A
456A
379A",
    "126384"
);
