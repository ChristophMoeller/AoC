use std::collections::{HashMap, VecDeque};

use super::*;
use crate::utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone)]
struct HikingGraph {
    paths: Vec<(u64, usize)>,
    junctions: Vec<Vec<usize>>,
    start: usize,
    end: usize,
}

fn longest_path(graph: &HikingGraph) -> u64 {
    let mut max_so_far = 0;

    let mut visited = vec![false; graph.junctions.len()];

    fn internal(
        graph: &HikingGraph,
        visited: &mut [bool],
        max_so_far: &mut u64,
        so_far: u64,
        junction: usize,
    ) {
        if junction == graph.end {
            *max_so_far = (*max_so_far).max(so_far);
        }

        visited[junction] = true;

        for &path_id in &graph.junctions[junction] {
            let (length, dest) = graph.paths[path_id];

            if !visited[dest] {
                internal(graph, visited, max_so_far, so_far + length, dest);
            }
        }

        visited[junction] = false;
    }

    internal(graph, &mut visited, &mut max_so_far, 0, graph.start);

    max_so_far
}

fn to_graph(grid: &Grid<Tile>) -> HikingGraph {
    let mut visited = Grid::new(grid.width() as usize, grid.height() as usize, |_, _| false);

    fn trace_path(
        grid: &Grid<Tile>,
        visited: &mut Grid<bool>,
        last: (isize, isize),
        current: (isize, isize),
        length: u64,
    ) -> (u64, (isize, isize), bool, bool) {
        if visited[current] {
            return (length, current, true, true);
        }

        let mut next = None;
        let mut is_junction = false;
        for ((x, y), t) in grid.neighbors4(current.0, current.1) {
            if t == &Tile::Forest || last == (x, y) {
                continue;
            }
            if next.is_none() {
                next = Some((x, y));
            } else {
                is_junction = true;
            }
        }

        if is_junction {
            return (length, current, true, true);
        }
        let Some(next) = next else {
            return (length, current, true, true);
        };

        visited[current] = true;
        let mut res = trace_path(grid, visited, current, next, length + 1);

        if let Tile::Slope(dir) = grid[current] {
            let forced_next = (current.0 + dir.offset().0, current.1 + dir.offset().1);
            if next == forced_next {
                res.3 = false;
            } else if last == forced_next {
                res.2 = false;
            } else {
                res.2 = false;
                res.3 = false;
            }
        }

        res
    }

    let start_position = (1isize, 0isize);
    let end_position = (grid.width() - 2, grid.height() - 1);

    let mut graph = HikingGraph {
        paths: Vec::new(),
        junctions: vec![vec![], vec![]],
        start: 0,
        end: 1,
    };
    let mut junctions = HashMap::new();
    junctions.insert(start_position, 0usize);
    junctions.insert(end_position, 1usize);
    let mut todo = VecDeque::new();
    todo.push_back(start_position);
    todo.push_back(end_position);

    while let Some(current) = todo.pop_front() {
        let id = junctions[&current];

        if let Tile::Slope(dir) = grid[current] {
            let forced_next = (current.0 + dir.offset().0, current.1 + dir.offset().1);
            if grid[forced_next] != Tile::Forest && !visited[forced_next] {
                let path = trace_path(&grid, &mut visited, current, forced_next, 1);
                let dest = *junctions.entry(path.1).or_insert_with(|| {
                    let junction_id = graph.junctions.len();
                    graph.junctions.push(vec![]);
                    todo.push_back(path.1);
                    junction_id
                });
                if path.2 {
                    let path_id = graph.paths.len();
                    graph.paths.push((path.0, dest));
                    graph.junctions[id].push(path_id);
                }
            }
        }

        for ((x, y), t) in grid.neighbors4(current.0, current.1) {
            if t != &Tile::Forest && !visited[(x, y)] {
                let path = trace_path(&grid, &mut visited, current, (x, y), 1);
                let dest = *junctions.entry(path.1).or_insert_with(|| {
                    let junction_id = graph.junctions.len();
                    graph.junctions.push(vec![]);
                    todo.push_back(path.1);
                    junction_id
                });
                if path.2 {
                    let path_id = graph.paths.len();
                    graph.paths.push((path.0, dest));
                    graph.junctions[id].push(path_id);
                }
                if path.3 {
                    let path_id = graph.paths.len();
                    graph.paths.push((path.0, id));
                    graph.junctions[dest].push(path_id);
                }
            }
        }
    }

    graph
}

pub struct Day23;
impl Solution for Day23 {
    type Input<'a> = Grid<Tile>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let graph = to_graph(&input);
        format!("{}", longest_path(&graph))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut without_slopes = input.clone();
        for y in 0..without_slopes.height() {
            for x in 0..without_slopes.width() {
                if let Tile::Slope(_) = without_slopes[(x, y)] {
                    without_slopes[(x, y)] = Tile::Path;
                }
            }
        }

        let graph = to_graph(&without_slopes);
        format!("{}", longest_path(&graph))
    }
}

mod parsing {
    use super::{Direction, Tile};
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> Grid<Tile> {
        Grid::parse(input, |c| match c {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '>' => Tile::Slope(Direction::E),
            '<' => Tile::Slope(Direction::W),
            '^' => Tile::Slope(Direction::N),
            'v' => Tile::Slope(Direction::S),
            _ => unimplemented!(),
        })
    }
}

gen_test!(
    a,
    Day23,
    r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
    "94"
);
gen_test!(
    b,
    Day23,
    r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
    "154"
);
