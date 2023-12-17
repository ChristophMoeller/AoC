use itertools::Itertools;

use super::*;
use crate::utils::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn loss<const MIN: usize, const MAX: usize>(blocks: &Grid<u32>) -> u32 {
    let mut loss: [[Grid<u32>; MAX]; 4] = core::array::from_fn(|_| {
        core::array::from_fn(|_| {
            Grid::<u32>::new(blocks.width() as usize, blocks.height() as usize, |_, _| {
                u32::MAX
            })
        })
    });

    loss[0][0][(0isize, 0isize)] = 0;
    loss[1][0][(0isize, 0isize)] = 0;
    loss[2][0][(0isize, 0isize)] = 0;
    loss[3][0][(0isize, 0isize)] = 0;

    loss[Direction::S as usize][0][(0isize, 1isize)] = blocks[(0isize, 1isize)];
    loss[Direction::E as usize][0][(1isize, 0isize)] = blocks[(1isize, 0isize)];

    loop {
        let mut changed = false;

        for s in 0..MAX {
            for y in 0..blocks.height() {
                for x in 0..blocks.width() {
                    for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
                        let offset = match d {
                            Direction::S => (0, -1),
                            Direction::N => (0, 1),
                            Direction::E => (-1, 0),
                            Direction::W => (1, 0),
                        };
                        let ox = x + offset.0;
                        let oy = y + offset.1;
                        let vert_dirs = if d == Direction::N || d == Direction::S {
                            [Direction::E, Direction::W]
                        } else {
                            [Direction::N, Direction::S]
                        };

                        let res = if s > 0 {
                            loss[d as usize][s - 1].get(ox, oy)
                        } else {
                            vert_dirs
                                .iter()
                                .cartesian_product(MIN..MAX)
                                .filter_map(|(&d, s)| loss[d as usize][s].get(ox, oy))
                                .min()
                        }
                        .copied()
                        .unwrap_or(u32::MAX)
                        .saturating_add(blocks[(x, y)]);

                        if res < loss[d as usize][s][(x, y)] {
                            changed = true;
                            loss[d as usize][s][(x, y)] = res;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    [Direction::N, Direction::E, Direction::S, Direction::W]
        .into_iter()
        .cartesian_product(MIN..MAX)
        .map(|(d, s)| loss[d as usize][s][(blocks.width() - 1, blocks.height() - 1)])
        .min()
        .unwrap()
}

pub struct Day17;
impl Solution for Day17 {
    type Input<'a> = Grid<u32>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let res = loss::<0, 3>(input);
        format!("{}", res)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let res = loss::<3, 10>(input);
        format!("{}", res)
    }
}

mod parsing {
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> Grid<u32> {
        Grid::parse(input, |c| c.to_digit(10).unwrap())
    }
}

gen_test!(
    a,
    Day17,
    r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    "102"
);
gen_test!(
    b,
    Day17,
    r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    "94"
);
