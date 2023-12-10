use std::f64::consts::PI;

use super::*;
use crate::utils::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    NS,
    WE,
    NW,
    NE,
    SW,
    SE,
    Start,
    Empty,
}

impl Segment {
    fn open_ends(&self) -> Option<[(isize, isize); 2]> {
        match self {
            Self::NS => Some([(0, -1), (0, 1)]),
            Self::WE => Some([(-1, 0), (1, 0)]),
            Self::NW => Some([(0, -1), (-1, 0)]),
            Self::NE => Some([(0, -1), (1, 0)]),
            Self::SW => Some([(0, 1), (-1, 0)]),
            Self::SE => Some([(0, 1), (1, 0)]),
            _ => None,
        }
    }
}

pub struct Day10;
impl Solution for Day10 {
    type Input<'a> = ((isize, isize), Grid<Segment>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut distances: Grid<_> =
            vec![vec![u32::MAX; input.1.width() as usize]; input.1.height() as usize].into();

        fn walk(segments: &Grid<Segment>, distances: &mut Grid<u32>, x: isize, y: isize, d: u32) {
            if distances[(x, y)] < d {
                return;
            }
            let s = segments[(x, y)];
            distances[(x, y)] = d;

            for offset in s.open_ends().unwrap() {
                walk(segments, distances, x + offset.0, y + offset.1, d + 1)
            }
        }

        distances[input.0] = 0;
        for ((x, y), s) in input.1.neighbors4(input.0 .0, input.0 .1) {
            if let Some(oe) = s.open_ends() {
                if oe
                    .iter()
                    .any(|(dx, dy)| input.0 .0 == x + dx && input.0 .1 == y + dy)
                {
                    walk(&input.1, &mut distances, x, y, 1);
                }
            }
        }

        let max = distances
            .into_iter()
            .map(|(_, d)| *d)
            .filter(|&d| d != u32::MAX)
            .max()
            .unwrap();

        format!("{}", max)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut visited: Grid<_> =
            vec![vec![false; input.1.width() as usize]; input.1.height() as usize].into();

        fn walk_main_loop(
            segments: &Grid<Segment>,
            visited: &mut Grid<bool>,
            main_loop: &mut Vec<(isize, isize)>,
            x: isize,
            y: isize,
        ) {
            if visited[(x, y)] {
                return;
            }
            let s = segments[(x, y)];
            visited[(x, y)] = true;
            main_loop.push((x, y));

            for offset in s.open_ends().unwrap() {
                walk_main_loop(segments, visited, main_loop, x + offset.0, y + offset.1);
            }
        }

        let mut main_loop = Vec::new();
        visited[input.0] = true;
        main_loop.push(input.0);
        for ((x, y), s) in input.1.neighbors4(input.0 .0, input.0 .1) {
            if let Some(oe) = s.open_ends() {
                if oe
                    .iter()
                    .any(|(dx, dy)| input.0 .0 == x + dx && input.0 .1 == y + dy)
                {
                    walk_main_loop(&input.1, &mut visited, &mut main_loop, x, y);
                }
            }
        }

        let mut components: Grid<_> =
            vec![vec![u32::MAX; input.1.width() as usize]; input.1.height() as usize].into();

        let mut component_counter = 0;

        for y in 0..components.height() {
            for x in 0..components.width() {
                if !visited[(x, y)] {
                    let c = components.neighbors8(x, y).map(|(_, c)| *c).min().unwrap();
                    if c == u32::MAX {
                        components[(x, y)] = component_counter;
                        component_counter += 1;
                    } else {
                        components[(x, y)] = c;
                    }
                }
            }
        }

        let mut equiv_components = petgraph::unionfind::UnionFind::new(component_counter as usize);
        for ((x, y), &c) in &components {
            for (_, &oc) in components.neighbors8(x, y) {
                if c != u32::MAX && oc != u32::MAX {
                    equiv_components.union(c, oc);
                }
            }
        }

        fn winding_number(main_loop: &Vec<(isize, isize)>, x: isize, y: isize) -> i32 {
            use itertools::Itertools;

            let angle_sum = main_loop
                .iter()
                .cycle()
                .tuple_windows()
                .take(main_loop.len())
                .map(|((a0, a1), (b0, b1))| {
                    let a0 = (a0 - x) as f64;
                    let a1 = (a1 - y) as f64;
                    let b0 = (b0 - x) as f64;
                    let b1 = (b1 - y) as f64;

                    let al = (a0 * a0 + a1 * a1).sqrt();
                    let bl = (b0 * b0 + b1 * b1).sqrt();

                    let (a0, a1) = (-a1 / al, a0 / al);
                    let (b0, b1) = (b0 / bl, b1 / bl);

                    let d = a0 * b0 + a1 * b1;

                    d.acos() - PI / 2.0
                })
                .sum::<f64>();

            if angle_sum > 0.0 {
                ((angle_sum / (PI * 2.0)) + 0.5) as i32
            } else if angle_sum < 0.0 {
                ((angle_sum / (PI * 2.0)) - 0.5) as i32
            } else {
                0
            }
        }

        let mut winding_numbers = vec![None; component_counter as usize];
        let mut nodes_in_component = vec![0; component_counter as usize];

        for y in 0..components.height() {
            for x in 0..components.width() {
                if components[(x, y)] == u32::MAX {
                    continue;
                }
                let id = equiv_components.find(components[(x, y)]) as usize;
                nodes_in_component[id] += 1;
                if winding_numbers[id].is_none() {
                    winding_numbers[id] = Some(winding_number(&main_loop, x, y));
                }
            }
        }

        let inside: usize = winding_numbers
            .iter()
            .zip(nodes_in_component.iter())
            .filter_map(|(&wn, &c)| {
                let wn = wn?;
                if wn != 0 {
                    Some(c)
                } else {
                    None
                }
            })
            .sum();

        format!("{}", inside)
    }
}

mod parsing {
    use super::Segment;
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> ((isize, isize), Grid<Segment>) {
        use Segment::*;
        let grid = Grid::parse(input, |c| match c {
            '|' => NS,
            '-' => WE,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Empty,
            'S' => Start,
            _ => unimplemented!(),
        });

        let ((sx, sy), _) = grid.into_iter().find(|((_, _), &s)| s == Start).unwrap();

        ((sx, sy), grid)
    }
}

gen_test!(
    a,
    Day10,
    r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    "8"
);
gen_test!(
    b,
    Day10,
    r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    "10"
);
