use std::collections::HashSet;

use itertools::Itertools;

use super::*;

pub struct Day10;
impl Solution for Day10 {
    type Input<'a> = Vec<Vec<u32>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
            .lines()
            .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect_vec())
            .collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut reachable = input
            .iter()
            .map(|line| line.iter().map(|_| HashSet::new()).collect_vec())
            .collect_vec();

        for (x, line) in input.iter().enumerate() {
            for (y, h) in line.iter().enumerate() {
                if *h == 9 {
                    reachable[x][y].insert((x, y));
                }
            }
        }

        for i in (0..=8).rev() {
            for (x, line) in input.iter().enumerate() {
                for (y, h) in line.iter().enumerate() {
                    if *h == i {
                        let (xl, xg) = reachable.split_at_mut(x);
                        let (xc, xg) = xg.split_first_mut().unwrap();
                        let (yl, yg) = xc.split_at_mut(y);
                        let (yc, yg) = yg.split_first_mut().unwrap();
                        if x > 0 && input[x - 1][y] == *h + 1 {
                            yc.extend(xl.last().unwrap()[y].iter());
                        }
                        if y > 0 && input[x][y - 1] == *h + 1 {
                            yc.extend(yl.last().unwrap().iter());
                        }
                        if x < input.len() - 1 && input[x + 1][y] == *h + 1 {
                            yc.extend(xg.first().unwrap()[y].iter());
                        }
                        if y < input.len() - 1 && input[x][y + 1] == *h + 1 {
                            yc.extend(yg.first().unwrap().iter());
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        for (x, line) in reachable.iter().enumerate() {
            for (y, r) in line.iter().enumerate() {
                if input[x][y] == 0 {
                    sum += r.len();
                }
            }
        }

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut possibilities = input
            .iter()
            .map(|line| line.iter().map(|_| 0).collect_vec())
            .collect_vec();

        for (x, line) in input.iter().enumerate() {
            for (y, h) in line.iter().enumerate() {
                if *h == 9 {
                    possibilities[x][y] = 1;
                }
            }
        }

        for i in (0..=8).rev() {
            for (x, line) in input.iter().enumerate() {
                for (y, h) in line.iter().enumerate() {
                    if *h == i {
                        if x > 0 && input[x - 1][y] == *h + 1 {
                            possibilities[x][y] += possibilities[x - 1][y];
                        }
                        if y > 0 && input[x][y - 1] == *h + 1 {
                            possibilities[x][y] += possibilities[x][y - 1];
                        }
                        if x < input.len() - 1 && input[x + 1][y] == *h + 1 {
                            possibilities[x][y] += possibilities[x + 1][y];
                        }
                        if y < input.len() - 1 && input[x][y + 1] == *h + 1 {
                            possibilities[x][y] += possibilities[x][y + 1];
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        for (x, line) in possibilities.iter().enumerate() {
            for (y, r) in line.iter().enumerate() {
                if input[x][y] == 0 {
                    sum += r;
                }
            }
        }

        format!("{}", sum)
    }
}

gen_test!(
    a,
    Day10,
    r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    "36"
);
gen_test!(
    b,
    Day10,
    r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    "81"
);
