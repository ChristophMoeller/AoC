use itertools::Itertools;
use regex::Regex;

use super::*;

pub struct Day14;
impl Solution for Day14 {
    type Input<'a> = Vec<((i64, i64), (i64, i64))>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

        re.captures_iter(content)
            .map(|c| c.extract())
            .map(|(_, [px, py, vx, vy])| {
                (
                    (px.parse().unwrap(), py.parse().unwrap()),
                    (vx.parse().unwrap(), vy.parse().unwrap()),
                )
            })
            .collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let width = 101;
        let height = 103;

        let final_positions = advance_robots(&input, width, height, 100);
        let mut counter = [[0, 0], [0, 0]];
        for (x, y) in final_positions {
            let x_idx = if x < width / 2 {
                0
            } else if x > width / 2 {
                1
            } else {
                continue;
            };
            let y_idx = if y < height / 2 {
                0
            } else if y > height / 2 {
                1
            } else {
                continue;
            };
            counter[x_idx][y_idx] += 1;
        }

        format!(
            "{}",
            counter[0][0] * counter[0][1] * counter[1][0] * counter[1][1]
        )
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        for seconds in 0.. {
            let mut grid = [[false; 101]; 103];
            let positions = advance_robots(&input, 101, 103, seconds);
            for pos in &positions {
                grid[pos.1 as usize][pos.0 as usize] = true;
            }
            // Quick Entropy Check
            // A Christmas tree should have a low score
            // The area is divided into 10 x 10 small sections and we count the number of robots in
            // each segment. If the robots are gathered in one area, then only a few sectors
            // should have a lot of robots in them.
            let score: u64 = downsample::<10, 10>(&positions, 101, 103)
                .iter()
                .map(|line| line.iter().map(|&x| (x + 1).ilog2()).sum::<u32>() as u64)
                .sum();
            if score < 160 {
                println!("========== {} seconds ==========", seconds);
                println!(
                    "{}",
                    grid.map(|line| { line.map(|x| if x { "#" } else { " " }).join("") })
                        .join("\n")
                );
                println!("");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }

        format!("{}", 0)
    }
}

fn advance_robots(
    robots: &[((i64, i64), (i64, i64))],
    width: i64,
    height: i64,
    seconds: i64,
) -> Vec<(i64, i64)> {
    robots
        .iter()
        .map(|((px, py), (vx, vy))| {
            (
                (px + vx * seconds).rem_euclid(width),
                (py + vy * seconds).rem_euclid(height),
            )
        })
        .collect_vec()
}

fn downsample<const W: usize, const H: usize>(
    positions: &[(i64, i64)],
    width: i64,
    height: i64,
) -> [[u64; H]; W] {
    let mut counter = [[0; H]; W];
    for &(x, y) in positions {
        counter[x as usize * W / width as usize][y as usize * H / height as usize] += 1;
    }
    counter
}

gen_test!(
    a,
    Day14,
    r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    "21"
);
