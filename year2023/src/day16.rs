use std::collections::VecDeque;

use super::*;
use crate::utils::Grid;

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    SplitterV,
    SplitterH,
    MirrorNW,
    MirrorNE,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match *self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
        }
    }
}

impl Tile {
    fn bounce(&self, direction: Direction) -> [Option<Direction>; 2] {
        match *self {
            Tile::Empty => [Some(direction), None],
            Tile::SplitterV => match direction {
                Direction::E | Direction::W => [Some(Direction::N), Some(Direction::S)],
                Direction::N | Direction::S => [Some(direction), None],
            },
            Tile::SplitterH => match direction {
                Direction::N | Direction::S => [Some(Direction::E), Some(Direction::W)],
                Direction::E | Direction::W => [Some(direction), None],
            },
            Tile::MirrorNW => match direction {
                Direction::N => [Some(Direction::E), None],
                Direction::W => [Some(Direction::S), None],
                Direction::S => [Some(Direction::W), None],
                Direction::E => [Some(Direction::N), None],
            },
            Tile::MirrorNE => match direction {
                Direction::N => [Some(Direction::W), None],
                Direction::E => [Some(Direction::S), None],
                Direction::S => [Some(Direction::E), None],
                Direction::W => [Some(Direction::N), None],
            },
        }
    }
}

fn trace_path(
    input: &Grid<Tile>,
    position: (isize, isize),
    direction: Direction,
) -> [Grid<bool>; 4] {
    let rays = vec![vec![false; input.width() as usize]; input.height() as usize];
    let mut rays: [Grid<bool>; 4] = [
        rays.clone().into(),
        rays.clone().into(),
        rays.clone().into(),
        rays.clone().into(),
    ];

    rays[direction as usize][position] = true;
    let mut todo = VecDeque::new();
    todo.push_back((direction, position));

    while let Some((dir, pos)) = todo.pop_front() {
        for new_dir in input[pos].bounce(dir).into_iter().flatten() {
            let offset = new_dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if rays[new_dir as usize].get(new_pos.0, new_pos.1) == Some(&false) {
                rays[new_dir as usize][new_pos] = true;
                todo.push_back((new_dir, new_pos));
            }
        }
    }

    rays
}

#[allow(dead_code)]
fn print(input: &Grid<Tile>, rays: &[Grid<bool>; 4]) {
    for y in 0..input.height() {
        for x in 0..input.width() {
            if rays[0][(x, y)] {
                print!("^");
            } else if rays[1][(x, y)] {
                print!(">");
            } else if rays[2][(x, y)] {
                print!("v");
            } else if rays[3][(x, y)] {
                print!("<");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub struct Day16;
impl Solution for Day16 {
    type Input<'a> = Grid<Tile>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let rays = trace_path(input, (0, 0), Direction::E);

        let count = input
            .into_iter()
            .filter(|&(p, _)| rays[0][p] || rays[1][p] || rays[2][p] || rays[3][p])
            .count();

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let top = (0..input.width()).map(|x| (Direction::S, (x, 0)));
        let bottom = (0..input.width()).map(|x| (Direction::N, (x, input.height() - 1)));
        let left = (0..input.height()).map(|y| (Direction::E, (0, y)));
        let right = (0..input.height()).map(|y| (Direction::W, (input.width() - 1, y)));

        let max_count = top
            .chain(bottom)
            .chain(left)
            .chain(right)
            .map(|(dir, pos)| {
                let rays = trace_path(input, pos, dir);

                input
                    .into_iter()
                    .filter(|&(p, _)| rays[0][p] || rays[1][p] || rays[2][p] || rays[3][p])
                    .count()
            })
            .max()
            .unwrap();

        format!("{}", max_count)
    }
}

mod parsing {
    use crate::utils::Grid;

    use super::Tile;

    pub(super) fn parse(input: &str) -> Grid<Tile> {
        Grid::parse(input, |c| match c {
            '.' => Tile::Empty,
            '|' => Tile::SplitterV,
            '-' => Tile::SplitterH,
            '/' => Tile::MirrorNW,
            '\\' => Tile::MirrorNE,
            _ => unreachable!(),
        })
    }
}

gen_test!(
    a,
    Day16,
    r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    "46"
);
gen_test!(
    b,
    Day16,
    r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    "51"
);
