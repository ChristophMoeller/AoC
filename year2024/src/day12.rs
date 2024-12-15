use itertools::Itertools;

use super::*;

pub struct Day12;
impl Solution for Day12 {
    type Input<'a> = Vec<Vec<char>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let cc = get_components(input);

        let mut cost = 0;
        for x in 0..(input.len() * input[0].len()) {
            let (a, b) = get_fence_area(&cc, x as u32);
            cost += a * b;
        }

        format!("{}", cost)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let cc = get_components(input);
        let cct = transpose(&cc);

        let mut cost = 0;
        for x in 0..(input.len() * input[0].len()) {
            let (_, b) = get_fence_area(&cc, x as u32);
            let c = get_discount_fence_price(&cc, x as u32);
            let d = get_discount_fence_price(&cct, x as u32);
            cost += b * (c + d);
        }

        format!("{}", cost)
    }
}

fn transpose(cc: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    (0..cc[0].len())
        .map(|y| (0..cc.len()).map(|x| cc[x][y]).collect_vec())
        .collect_vec()
}

fn get_discount_fence_price(cc: &Vec<Vec<u32>>, t: u32) -> u64 {
    let mut fence = 0;
    for x in 0..(cc.len() - 1) {
        let mut prev_u = false;
        let mut prev_d = false;
        for (&a, &b) in cc[x].iter().zip(cc[x + 1].iter()) {
            if a != b {
                if a == t {
                    if !prev_u {
                        fence += 1;
                        prev_u = true;
                    }
                } else {
                    prev_u = false;
                }
                if b == t {
                    if !prev_d {
                        fence += 1;
                        prev_d = true;
                    }
                } else {
                    prev_d = false;
                }
            } else {
                prev_u = false;
                prev_d = false;
            }
        }
    }
    fence
}

fn get_fence_area(cc: &Vec<Vec<u32>>, t: u32) -> (u64, u64) {
    let mut fence = 0;
    let mut area = 0;
    for (x, r) in cc.iter().enumerate() {
        for (y, &c) in r.iter().enumerate() {
            if c != t {
                continue;
            }
            area += 1;
            if cc[x - 1][y] != c {
                fence += 1;
            }
            if cc[x + 1][y] != c {
                fence += 1;
            }
            if cc[x][y - 1] != c {
                fence += 1;
            }
            if cc[x][y + 1] != c {
                fence += 1;
            }
        }
    }
    (fence, area)
}

fn get_components(input: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut id = 0;
    let mut cc = vec![vec![u32::MAX; input[0].len() + 2]];
    cc.extend(input.iter().map(move |line| {
        let mut iid = id;
        id += input.len() as u32;
        let mut row = vec![u32::MAX];
        row.extend(line.iter().map(move |_| {
            let cid = iid;
            iid += 1;
            cid
        }));
        row.push(u32::MAX);
        row
    }));
    cc.push(vec![u32::MAX; input[0].len() + 2]);

    let mut changed = true;
    while changed {
        changed = false;
        for x in 0..input.len() {
            for y in 0..input[0].len() {
                if x > 0 && input[x - 1][y] == input[x][y] && cc[x][y + 1] < cc[x + 1][y + 1] {
                    cc[x + 1][y + 1] = cc[x][y + 1];
                    changed = true;
                } else if y > 0 && input[x][y - 1] == input[x][y] && cc[x + 1][y] < cc[x + 1][y + 1]
                {
                    cc[x + 1][y + 1] = cc[x + 1][y];
                    changed = true;
                } else if y < input[0].len() - 1
                    && input[x][y + 1] == input[x][y]
                    && cc[x + 1][y + 2] < cc[x + 1][y + 1]
                {
                    cc[x + 1][y + 1] = cc[x + 1][y + 2];
                    changed = true;
                } else if x < input.len() - 1
                    && input[x + 1][y] == input[x][y]
                    && cc[x + 2][y + 1] < cc[x + 1][y + 1]
                {
                    cc[x + 1][y + 1] = cc[x + 2][y + 1];
                    changed = true;
                }
            }
        }
    }

    cc
}

gen_test!(
    a,
    Day12,
    r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    "1930"
);
gen_test!(
    b,
    Day12,
    r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    "1206"
);
