use std::collections::HashMap;

use super::*;
use crate::utils::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Round,
    Cube,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Empty => {
                write!(f, ".")?;
            }
            Tile::Round => {
                write!(f, "O")?;
            }
            Tile::Cube => {
                write!(f, "#")?;
            }
        }
        Ok(())
    }
}

enum Direction {
    N,
    E,
    S,
    W,
}

fn calc_load(tiles: &Grid<Tile>) -> u64 {
    fn calc_load_internal(tiles: &Grid<Tile>, row: isize, col: isize, load: u64) -> u64 {
        if row >= tiles.height() {
            return 0;
        }
        match tiles[(col, row)] {
            Tile::Round => load + calc_load_internal(tiles, row + 1, col, load - 1),
            Tile::Cube => {
                calc_load_internal(tiles, row + 1, col, (tiles.height() - row) as u64 - 1)
            }
            Tile::Empty => calc_load_internal(tiles, row + 1, col, load),
        }
    }

    (0..tiles.width())
        .map(|col| calc_load_internal(tiles, 0, col, tiles.height() as u64))
        .sum()
}

fn cube_rocks(tiles: &Grid<Tile>) -> Vec<(isize, isize)> {
    tiles
        .into_iter()
        .filter(|(_, &x)| x == Tile::Cube)
        .map(|((x, y), _)| (x, y))
        .chain((0..tiles.height()).map(|y| (-1, y)))
        .chain((0..tiles.height()).map(|y| (tiles.width(), y)))
        .chain((0..tiles.width()).map(|x| (x, -1)))
        .chain((0..tiles.width()).map(|x| (x, tiles.height())))
        .collect()
}

fn tilt(
    tiles: &mut Grid<Tile>,
    cube_rocks: &Vec<(isize, isize)>,
    direction: &Direction,
) -> Vec<u8> {
    let mut load = vec![0u8; cube_rocks.len()];

    let d = match direction {
        &Direction::N => (0, 1),
        &Direction::S => (0, -1),
        &Direction::E => (-1, 0),
        &Direction::W => (1, 0),
    };

    for (i, &(x, y)) in cube_rocks.iter().enumerate() {
        let mut current = (x + d.0, y + d.1);
        loop {
            let Some(&tile) = tiles.get(current.0, current.1) else {
                break;
            };
            match tile {
                Tile::Cube => {
                    break;
                }
                Tile::Round => {
                    load[i] += 1;
                    tiles[current] = Tile::Empty;
                }
                _ => {}
            }
            current.0 += d.0;
            current.1 += d.1;
        }
        current = (x + d.0, y + d.1);
        for _ in 0..load[i] {
            tiles[current] = Tile::Round;
            current.0 += d.0;
            current.1 += d.1;
        }
    }

    load
}

fn spin_cycle(tiles: &mut Grid<Tile>, cycles: u64) {
    let mut cache = HashMap::new();

    let directions = [Direction::N, Direction::W, Direction::S, Direction::E];

    let cubes = cube_rocks(tiles);

    let mut i = 0u64;
    loop {
        i += 1;

        let mut k = Vec::new();
        for d in &directions {
            k = tilt(tiles, &cubes, d);
        }

        let &mut j = cache.entry(k).or_insert(i);
        if i != j {
            let diff = i - j;
            let rem = cycles - i;
            i += (rem / diff) * diff;
        }

        if i == cycles {
            break;
        }
    }
}

pub struct Day14;
impl Solution for Day14 {
    type Input<'a> = Grid<Tile>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let load = calc_load(input);
        format!("{}", load)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut input = input.clone();
        spin_cycle(&mut input, 1000000000);

        let load = input
            .into_iter()
            .filter_map(|((_, y), &t)| {
                if t == Tile::Empty || t == Tile::Cube {
                    None
                } else {
                    Some((input.height() - y) as u64)
                }
            })
            .sum::<u64>();

        format!("{}", load)
    }
}

mod parsing {
    use super::Tile;
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> Grid<Tile> {
        Grid::parse(input, |c| match c {
            'O' => Tile::Round,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => unimplemented!(),
        })
    }
}

gen_test!(
    a,
    Day14,
    r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    "136"
);
gen_test!(
    b,
    Day14,
    r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    "64"
);
