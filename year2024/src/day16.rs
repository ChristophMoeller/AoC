use std::cmp::Reverse;

use itertools::Itertools;
use priority_queue::PriorityQueue;
use utils::Grid;

use super::*;

pub struct Day16;
impl Solution for Day16 {
    type Input<'a> = (Grid<bool>, (isize, isize), (isize, isize));

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let mut end = (0isize, 0isize);
        let mut start = (0isize, 0isize);
        let raw_grid = content
            .lines()
            .enumerate()
            .map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == 'E' {
                            end = (i as isize, j as isize);
                            true
                        } else if c == 'S' {
                            start = (i as isize, j as isize);
                            true
                        } else if c == '.' {
                            true
                        } else {
                            false
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        (Grid::from(raw_grid), start, end)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let (map, start, end) = input;
        let dist = dist_map(map, end);

        format!("{}", dist[*start].iter().min().unwrap())
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let (map, start, end) = input;
        let dist = dist_map(map, end);

        let min_dist = *dist[*start].iter().min().unwrap();

        let mut part_of_fastest_path =
            Grid::new(map.width() as usize, map.height() as usize, |x, y| {
                if *start != (x, y) {
                    [false; 4]
                } else {
                    core::array::from_fn(|i| dist[*start][i] == min_dist)
                }
            });

        // Dirty solution, not reworked yet.
        // Maybe change this later to increase performance (even though it's not necessary)
        let mut changed = true;
        while changed {
            changed = false;
            for ((x, y), &s) in map {
                if !s {
                    continue;
                }
                for (i, &dir) in DIRECTIONS.iter().enumerate() {
                    if let Some(d) = dist.get(x + dir.0, y + dir.1) {
                        if part_of_fastest_path[(x, y)][i] && dist[(x, y)][i] == d[i] + 1 {
                            changed |= !part_of_fastest_path[(x + dir.0, y + dir.1)][i];
                            part_of_fastest_path[(x + dir.0, y + dir.1)][i] = true;
                        }
                    }
                }
                for (i, j) in (0..4).zip(1..5).map(|(i, j)| (i, j % 4)) {
                    if part_of_fastest_path[(x, y)][i] && dist[(x, y)][i] == dist[(x, y)][j] + 1000
                    {
                        changed |= !part_of_fastest_path[(x, y)][j];
                        part_of_fastest_path[(x, y)][j] = true;
                    }
                    if part_of_fastest_path[(x, y)][j] && dist[(x, y)][j] == dist[(x, y)][i] + 1000
                    {
                        changed |= !part_of_fastest_path[(x, y)][i];
                        part_of_fastest_path[(x, y)][i] = true;
                    }
                }
            }
        }

        format!(
            "{}",
            part_of_fastest_path
                .into_iter()
                .filter(|(_, &[a, b, c, d])| a || b || c || d)
                .count()
        )
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (-1isize, 0isize),
    (0isize, 1isize),
    (1isize, 0isize),
    (0isize, -1isize),
];

fn dist_map(map: &Grid<bool>, end: &(isize, isize)) -> Grid<[u64; 4]> {
    let mut dist = Grid::<[u64; 4]>::new(map.width() as usize, map.height() as usize, |x, y| {
        if (x, y) == *end {
            [0; 4]
        } else {
            [u64::MAX - 1000; 4]
        }
    });

    let mut changed = PriorityQueue::new();
    changed.push_increase((*end, 0usize), Reverse(0));
    changed.push_increase((*end, 1usize), Reverse(0));
    changed.push_increase((*end, 2usize), Reverse(0));
    changed.push_increase((*end, 3usize), Reverse(0));

    while let Some((((x, y), d), current_dist)) = changed.pop() {
        let (dx, dy) = DIRECTIONS[d];
        if map.get(x - dx, y - dy) == Some(&true) {
            if dist[(x - dx, y - dy)][d] > current_dist.0 + 1 {
                dist[(x - dx, y - dy)][d] = current_dist.0 + 1;
                changed.push_increase(((x - dx, y - dy), d), Reverse(current_dist.0 + 1));
            }
        }
        if dist[(x, y)][(d + 1) % 4] > current_dist.0 + 1000 {
            dist[(x, y)][(d + 1) % 4] = current_dist.0 + 1000;
            changed.push_increase(((x, y), (d + 1) % 4), Reverse(current_dist.0 + 1000));
        }
        if dist[(x, y)][(d + 3) % 4] > current_dist.0 + 1000 {
            dist[(x, y)][(d + 3) % 4] = current_dist.0 + 1000;
            changed.push_increase(((x, y), (d + 3) % 4), Reverse(current_dist.0 + 1000));
        }
    }
    return dist;
}
