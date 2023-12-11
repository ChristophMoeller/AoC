use super::*;
use crate::utils::Grid;

fn measure_expanded(input: &Grid<bool>, expansion: u64) -> u64 {
    let mut xs = vec![false; input.width() as usize];
    let mut ys = vec![false; input.height() as usize];
    let mut gals = vec![];

    for ((x, y), g) in input {
        if *g {
            xs[x as usize] = true;
            ys[y as usize] = true;
            gals.push((x, y));
        }
    }

    let mut sum = 0;

    for (i, &gal1) in gals.iter().enumerate() {
        for &gal2 in &gals[(i + 1)..] {
            let xa = gal1.0.min(gal2.0) as usize;
            let ya = gal1.1.min(gal2.1) as usize;
            let xb = gal1.0.max(gal2.0) as usize;
            let yb = gal1.1.max(gal2.1) as usize;

            sum += &xs[xa..xb]
                .iter()
                .map(|g| if *g { 1 } else { expansion })
                .sum::<u64>();
            sum += &ys[ya..yb]
                .iter()
                .map(|g| if *g { 1 } else { expansion })
                .sum::<u64>();
        }
    }

    sum
}

pub struct Day11;
impl Solution for Day11 {
    type Input<'a> = Grid<bool>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        format!("{}", measure_expanded(input, 2))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        format!("{}", measure_expanded(input, 1000000))
    }
}

mod parsing {
    use crate::utils::Grid;

    pub(super) fn parse(input: &str) -> Grid<bool> {
        Grid::parse(input, |c| match c {
            '.' => false,
            '#' => true,
            _ => unimplemented!(),
        })
    }
}

gen_test!(
    a,
    Day11,
    r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
    "374"
);
