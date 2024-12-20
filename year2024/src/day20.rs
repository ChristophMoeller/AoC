use itertools::Itertools;
use utils::Grid;

use super::*;

pub struct Day20;
impl Solution for Day20 {
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
        let path = find_path(&input.0, &input.1, &input.2);
        format!("{}", count_cheat_possibilities(&path, 2, 100))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let path = find_path(&input.0, &input.1, &input.2);
        format!("{}", count_cheat_possibilities(&path, 20, 100))
    }
}

fn count_cheat_possibilities(
    path: &[(isize, isize)],
    cheat_duration: usize,
    min_time_safe: usize,
) -> usize {
    path.iter()
        .enumerate()
        .map(|(i, (x, y))| {
            path[i..]
                .iter()
                .enumerate()
                .filter(|(j, (tx, ty))| {
                    let duration = ((tx - x).abs() + (ty - y).abs()) as usize;
                    if duration > cheat_duration {
                        return false;
                    }
                    *j >= min_time_safe + duration
                })
                .count()
        })
        .sum()
}

fn find_path(
    grid: &Grid<bool>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> Vec<(isize, isize)> {
    let mut path = Vec::new();

    let mut pos = *start;
    path.push(pos);
    while pos != *end {
        for (npos, &b) in grid.neighbors4(pos.0, pos.1) {
            if b && !path.iter().rev().take(3).contains(&npos) {
                pos = npos;
                path.push(pos);
                continue;
            }
        }
    }
    path
}
