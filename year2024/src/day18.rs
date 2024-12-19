use std::collections::BinaryHeap;

use itertools::Itertools;
use union_find::{QuickUnionUf, UnionByRank, UnionFind};
use utils::Grid;

use super::*;

pub struct Day18;
impl Solution for Day18 {
    type Input<'a> = Vec<(isize, isize)>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let (size, byte_count) = if cfg!(not(test)) {
            (70usize, 1024usize)
        } else {
            (6usize, 12usize)
        };

        let mut grid = Grid::new(size + 1, size + 1, |_, _| false);
        for &(x, y) in input.iter().take(byte_count) {
            grid[(x, y)] = true;
        }

        let mut dist = Grid::new(size + 1, size + 1, |_, _| u32::MAX);
        dist[(size, size)] = 0;
        let mut queue = BinaryHeap::new();
        queue.push((0, size as isize, size as isize));

        while dist[(0isize, 0isize)] == u32::MAX {
            let (_, x, y) = queue.pop().unwrap();
            let d = dist[(x, y)];
            for (nx, ny) in dist.neighborpos4(x, y) {
                let nd = dist[(nx, ny)];
                if !grid[(nx, ny)] && nd > d + 1 {
                    dist[(nx, ny)] = d + 1;
                    queue.push((-nx - ny - (d as isize) - 1, nx, ny))
                }
            }
        }

        format!("{}", dist[(0isize, 0isize)])
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let size = if cfg!(not(test)) { 70usize } else { 6usize };
        let idx = |x, y| x as usize + (size + 1) * y as usize;

        let mut grid = Grid::new(size + 1, size + 1, |_, _| false);
        for &(x, y) in input {
            grid[(x, y)] = true;
        }

        let mut components = QuickUnionUf::<UnionByRank>::new((size + 1) * (size + 1));
        for ((x, y), &b) in &grid {
            for ((nx, ny), &nb) in grid.neighbors4(x, y) {
                if !b && !nb {
                    components.union(idx(nx, ny), idx(x, y));
                }
            }
        }

        for &(x, y) in input.iter().rev() {
            grid[(x, y)] = false;
            for ((nx, ny), &nb) in grid.neighbors4(x, y) {
                if !nb {
                    components.union(idx(nx, ny), idx(x, y));
                }
            }

            if components.find(0) == components.find((size + 1) * (size + 1) - 1) {
                return format!("{},{}", x, y);
            }
        }

        unreachable!()
    }
}

gen_test!(
    a,
    Day18,
    r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    "22"
);

gen_test!(
    b,
    Day18,
    r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    "6,1"
);
