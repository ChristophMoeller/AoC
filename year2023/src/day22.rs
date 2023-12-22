use std::collections::BTreeSet;

use itertools::iproduct;

use crate::utils::Grid;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    lower: (i32, i32, i32),
    upper: (i32, i32, i32),
}

impl Brick {
    fn xy_cut(&self) -> impl Iterator<Item = (isize, isize)> {
        iproduct!(
            (self.lower.0 as isize)..=(self.upper.0 as isize),
            (self.lower.1 as isize)..=(self.upper.1 as isize)
        )
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.lower
                .2
                .cmp(&other.lower.2)
                .then_with(|| self.lower.0.cmp(&other.lower.0))
                .then_with(|| self.lower.1.cmp(&other.lower.1))
                .then_with(|| self.upper.2.cmp(&other.upper.2))
                .then_with(|| self.upper.0.cmp(&other.upper.0))
                .then_with(|| self.upper.1.cmp(&other.upper.1)),
        )
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_tower_area<'a>(bricks: impl IntoIterator<Item = &'a Brick>) -> (i32, i32, i32, i32) {
    bricks
        .into_iter()
        .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |acc, b| {
            (
                acc.0.min(b.lower.0),
                acc.1.min(b.lower.1),
                acc.2.max(b.upper.0),
                acc.3.max(b.upper.1),
            )
        })
}

fn simulate_fall(bricks: impl IntoIterator<Item = Brick>) -> Vec<Brick> {
    let mut bricks = Vec::from_iter(bricks.into_iter());
    if bricks.is_empty() {
        return bricks;
    }
    bricks.sort_unstable();

    let (min_x, min_y, max_x, max_y) = get_tower_area(&bricks);
    let mut heightmap = Grid::new(
        (max_x - min_x + 1) as usize,
        (max_y - min_y + 1) as usize,
        |_, _| 0,
    );
    heightmap.set_wrapping(true);

    for b in &mut bricks {
        let ground = b.xy_cut().map(|(x, y)| heightmap[(x, y)]).max().unwrap() + 1;

        b.upper.2 = ground + (b.upper.2 - b.lower.2);
        b.lower.2 = ground;

        for (x, y) in b.xy_cut() {
            heightmap[(x, y)] = b.upper.2;
        }
    }

    bricks
}

fn build_support_structure(bricks: &[Brick]) -> Vec<BTreeSet<usize>> {
    let (min_x, min_y, max_x, max_y) = get_tower_area(bricks);
    let mut heightmap = Grid::new(
        (max_x - min_x + 1) as usize,
        (max_y - min_y + 1) as usize,
        |_, _| (0i32, -1isize),
    );
    heightmap.set_wrapping(true);

    bricks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut support = BTreeSet::new();
            for (x, y) in b.xy_cut() {
                let (h, s) = heightmap[(x, y)];
                if s >= 0 && h == b.lower.2 - 1 {
                    support.insert(s as usize);
                }
                heightmap[(x, y)] = (b.upper.2, i as isize);
            }
            support
        })
        .collect()
}

fn evaluate_chain_reaction(
    support_structure: &[BTreeSet<usize>],
    removed_index: usize,
) -> BTreeSet<usize> {
    let mut fallen_bricks = BTreeSet::new();
    fallen_bricks.insert(removed_index);

    for i in (removed_index + 1)..support_structure.len() {
        let s = &support_structure[i];

        if !s.is_empty() && s.is_subset(&fallen_bricks) {
            fallen_bricks.insert(i);
        }
    }

    fallen_bricks.remove(&removed_index);
    fallen_bricks
}

pub struct Day22;
impl Solution for Day22 {
    type Input<'a> = Vec<Brick>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let fallen_bricks = simulate_fall(input.iter().copied());
        let support = build_support_structure(&fallen_bricks);

        let count = (0..support.len())
            .filter(|i| evaluate_chain_reaction(&support, *i).is_empty())
            .count();

        format!("{count}")
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let fallen_bricks = simulate_fall(input.iter().copied());
        let support = build_support_structure(&fallen_bricks);

        let sum = (0..support.len())
            .map(|i| {
                let cr = evaluate_chain_reaction(&support, i);
                cr.len()
            })
            .sum::<usize>();

        format!("{sum}")
    }
}

mod parsing {
    use super::Brick;

    pub(super) fn parse(input: &str) -> Vec<Brick> {
        input
            .lines()
            .filter_map(|line| {
                let parts: Vec<_> = line.split([',', '~']).collect();
                let &[x0, y0, z0, x1, y1, z1, ..] = parts.as_slice() else {
                    return None;
                };

                let x0 = x0.parse::<i32>().ok()?;
                let y0 = y0.parse::<i32>().ok()?;
                let z0 = z0.parse::<i32>().ok()?;
                let x1 = x1.parse::<i32>().ok()?;
                let y1 = y1.parse::<i32>().ok()?;
                let z1 = z1.parse::<i32>().ok()?;

                Some(Brick {
                    lower: (x0.min(x1), y0.min(y1), z0.min(z1)),
                    upper: (x0.max(x1), y0.max(y1), z0.max(z1)),
                })
            })
            .collect()
    }
}

gen_test!(
    a,
    Day22,
    r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    "5"
);
gen_test!(
    b,
    Day22,
    r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    "7"
);
