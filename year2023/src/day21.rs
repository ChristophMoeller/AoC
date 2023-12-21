use rayon::prelude::*;

use super::*;

use crate::utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    GardenPlot,
    Rock,
    Start,
}

fn perform_step(garden: &Grid<Tile>, positions: &Grid<bool>) -> Grid<bool> {
    let mut out: Grid<bool> =
        vec![vec![false; positions.width() as usize]; positions.height() as usize].into();

    for position in positions
        .into_iter()
        .filter_map(|(position, x)| if *x { Some(position) } else { None })
    {
        for ((nx, ny), t) in garden.neighbors4(position.0, position.1) {
            if t != &Tile::Rock {
                out[(nx, ny)] = true;
            }
        }
    }

    out
}

fn sim_with_start_pos(garden: &Grid<Tile>, start_pos: (isize, isize)) -> Vec<usize> {
    let mut pos: Grid<bool> =
        vec![vec![false; garden.width() as usize]; garden.height() as usize].into();
    pos.set_wrapping(true);
    pos[start_pos] = true;
    pos.set_wrapping(false);

    let mut res = Vec::new();
    res.push(1);

    loop {
        pos = perform_step(garden, &pos);
        let c = pos.into_iter().filter(|(_, &t)| t).count();
        if res.len() > 1 && res[res.len() - 2] == c {
            break;
        }
        res.push(c);
    }

    res
}

fn get_c_at_time(cs: &[usize], t: usize) -> usize {
    if t < cs.len() {
        return cs[t];
    }

    let mut idx = cs.len() - 1;
    if t % 2 != idx % 2 {
        idx -= 1;
    }

    cs[idx]
}

pub struct Day21;
impl Solution for Day21 {
    type Input<'a> = (Grid<Tile>, (isize, isize));

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let counts = sim_with_start_pos(&input.0, input.1);
        format!("{}", get_c_at_time(&counts, 64))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        const STEPS: isize = 26501365;

        let x = input.1 .0;
        let y = input.1 .1;

        let b = sim_with_start_pos(&input.0, input.1);

        let n = sim_with_start_pos(&input.0, (x, 0));
        let s = sim_with_start_pos(&input.0, (x, -1));
        let e = sim_with_start_pos(&input.0, (-1, y));
        let w = sim_with_start_pos(&input.0, (0, y));

        let ne = sim_with_start_pos(&input.0, (-1, 0));
        let nw = sim_with_start_pos(&input.0, (0, 0));
        let se = sim_with_start_pos(&input.0, (-1, -1));
        let sw = sim_with_start_pos(&input.0, (0, -1));

        let ext_x = STEPS / input.0.width() + 1;
        let ext_y = STEPS / input.0.height() + 1;

        let count: usize = ((-ext_x)..=ext_x)
            .into_par_iter()
            .map(|x| {
                let mut count = 0;
                for y in (-ext_y)..=ext_y {
                    let mut offset = 0;
                    if x != 0 {
                        offset += (x.abs() - 1) * input.0.width();
                        if x < 0 {
                            offset += input.1 .0 + 1;
                        }
                        if x > 0 {
                            offset += input.0.width() - input.1 .0;
                        }
                    }
                    if y != 0 {
                        offset += (y.abs() - 1) * input.0.height();
                        if y < 0 {
                            offset += input.1 .1 + 1;
                        }
                        if y > 0 {
                            offset += input.0.height() - input.1 .1;
                        }
                    }

                    if offset > STEPS {
                        continue;
                    }

                    let t = (STEPS - offset) as usize;

                    count += match (x.cmp(&0), y.cmp(&0)) {
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                            get_c_at_time(&se, t)
                        }
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                            get_c_at_time(&e, t)
                        }
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                            get_c_at_time(&ne, t)
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                            get_c_at_time(&s, t)
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                            get_c_at_time(&b, t)
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                            get_c_at_time(&n, t)
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                            get_c_at_time(&sw, t)
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                            get_c_at_time(&w, t)
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                            get_c_at_time(&nw, t)
                        }
                    };
                }
                count
            })
            .sum();

        format!("{count}")
    }
}

mod parsing {

    use super::Tile;
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> (Grid<Tile>, (isize, isize)) {
        let mut grid = Grid::parse(input, |c| match c {
            '.' => Tile::GardenPlot,
            'S' => Tile::Start,
            '#' => Tile::Rock,
            _ => unimplemented!(),
        });

        let start = grid
            .into_iter()
            .find_map(|((x, y), t)| {
                if t == &Tile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
            .unwrap();

        grid[start] = Tile::GardenPlot;

        (grid, start)
    }
}

gen_test!(
    b,
    Day21,
    r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    "16"
);
// gen_test!(b, Day21, r"", "");
