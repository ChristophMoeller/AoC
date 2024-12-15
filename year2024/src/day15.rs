use utils::Grid;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Box,
    BoxL,
    BoxR,
    Space,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tile::Box => "X",
                Tile::BoxL => "[",
                Tile::BoxR => "]",
                Tile::Wall => "█",
                Tile::Space => " ",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline(always)]
    fn offset(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub struct Day15;
impl Solution for Day15 {
    type Input<'a> = (Grid<Tile>, (isize, isize), Vec<Direction>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let (map, seq) = content.split_once("\n\n").unwrap();
        let (map, pos) = parse::map(map);
        let seq = parse::moves(seq);
        (map, pos, seq)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut map = input.0.clone();
        let mut pos = input.1;
        let seq = input.2.iter();
        for &dir in seq {
            move_robot(&mut map, &mut pos, dir);
        }

        let mut coord_sum = 0;
        for ((x, y), tile) in &map {
            if *tile == Tile::Box {
                coord_sum += x + y * 100;
            }
        }

        format!("{}", coord_sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let base_map = input.0.clone();
        let mut map = Grid::new(
            base_map.width() as usize * 2,
            base_map.height() as usize,
            |x, y| match (base_map[(x / 2, y)], x.rem_euclid(2)) {
                (Tile::Space, _) => Tile::Space,
                (Tile::Wall, _) => Tile::Wall,
                (Tile::Box, 0) => Tile::BoxL,
                (Tile::Box, 1) => Tile::BoxR,
                _ => unreachable!(),
            },
        );
        let pos = input.1;
        let mut pos = (pos.0 * 2, pos.1);
        let seq = input.2.iter();
        for &dir in seq {
            move_robot(&mut map, &mut pos, dir);
        }

        let mut coord_sum = 0;
        for ((x, y), tile) in &map {
            if *tile == Tile::BoxL {
                coord_sum += x + y * 100;
            }
        }

        format!("{}", coord_sum)
    }
}

fn move_robot(map: &mut Grid<Tile>, pos: &mut (isize, isize), dir: Direction) {
    if can_move(map, *pos, dir) {
        apply_force(map, *pos, dir);
        let (dx, dy) = dir.offset();
        *pos = (pos.0 + dx, pos.1 + dy);
    }
}

fn can_move(map: &Grid<Tile>, pos: (isize, isize), dir: Direction) -> bool {
    let (x, y) = pos;
    let (dx, dy) = dir.offset();
    match map[(x + dx, y + dy)] {
        Tile::Box => can_move(map, (x + dx, y + dy), dir),
        Tile::BoxL => {
            if dx == 0 {
                can_move(map, (x, y + dy), dir) && can_move(map, (x + 1, y + dy), dir)
            } else {
                can_move(map, (x + dx, y), dir)
            }
        }
        Tile::BoxR => {
            if dx == 0 {
                can_move(map, (x, y + dy), dir) && can_move(map, (x - 1, y + dy), dir)
            } else {
                can_move(map, (x + dx, y), dir)
            }
        }
        Tile::Space => true,
        Tile::Wall => false,
    }
}

fn apply_force(map: &mut Grid<Tile>, pos: (isize, isize), dir: Direction) {
    let (x, y) = pos;
    let (dx, dy) = dir.offset();
    match map[(x + dx, y + dy)] {
        Tile::Box => apply_force(map, (x + dx, y + dy), dir),
        Tile::BoxL => {
            if dx == 0 {
                apply_force(map, (x, y + dy), dir);
                apply_force(map, (x + 1, y + dy), dir);
            } else {
                apply_force(map, (x + dx, y), dir);
            }
        }
        Tile::BoxR => {
            if dx == 0 {
                apply_force(map, (x, y + dy), dir);
                apply_force(map, (x - 1, y + dy), dir);
            } else {
                apply_force(map, (x + dx, y), dir);
            }
        }
        _ => {}
    }
    map[(x + dx, y + dy)] = map[pos];
    map[pos] = Tile::Space
}

mod parse {
    use itertools::Itertools;

    use super::{utils::Grid, Direction, Tile};

    pub fn map(content: &str) -> (Grid<Tile>, (isize, isize)) {
        let mut pos = (1, 1);
        let raw_map = content
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == '#' {
                            Tile::Wall
                        } else if c == 'O' {
                            Tile::Box
                        } else if c == '@' {
                            pos = (j as isize, i as isize);
                            Tile::Space
                        } else {
                            Tile::Space
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        (Grid::from(raw_map), pos)
    }

    pub fn moves(content: &str) -> Vec<Direction> {
        content
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            })
            .collect_vec()
    }
}

gen_test!(
    a,
    Day15,
    r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    "10092"
);
gen_test!(
    b,
    Day15,
    r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    "9021"
);
