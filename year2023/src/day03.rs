use super::*;
use crate::utils::Grid;

// DISCLAIMER :D
// The current version of this solution was done at 6am.
// Therefore, it is definitly not the most beautiful way to solve the
// problem, but it works.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    None,
    Symbol(char),
    Digit(u32),
}

impl Entry {
    fn is_symbol(&self) -> bool {
        match self {
            Entry::Symbol(_) => true,
            _ => false,
        }
    }
    fn is_digit(&self) -> bool {
        match self {
            Entry::Digit(_) => true,
            _ => false,
        }
    }
}

pub struct Day03;
impl Solution for Day03 {
    type Input<'a> = Grid<Entry>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(grid: &Self::Input<'a>) -> String {
        let mut current = 0;
        let mut next_to_symbol = false;
        let mut sum = 0;

        for ((x, y), e) in grid {
            if x == 0 || !e.is_digit() {
                if next_to_symbol {
                    sum += current;
                }
                current = 0;
                next_to_symbol = false;
            }
            if let Entry::Digit(d) = e {
                current = current * 10 + d;
                if grid.neighbors8(x, y).any(|(_, e)| e.is_symbol()) {
                    next_to_symbol = true;
                }
            }
        }

        format!("{:#?}", sum)
    }

    fn part_b<'a>(grid: &Self::Input<'a>) -> String {
        fn find_number(
            grid: &Grid<Entry>,
            mut x: isize,
            y: isize,
        ) -> Option<((isize, isize), u32)> {
            let e = &grid.get(x, y)?;

            if !e.is_digit() {
                return None;
            }

            while &grid.get(x - 1, y).map(Entry::is_digit) == &Some(true) {
                x -= 1;
            }

            let start = (x, y);
            let mut value = 0;
            while let Some(&Entry::Digit(d)) = grid.get(x, y) {
                value = value * 10 + d;
                x += 1;
            }

            Some((start, value))
        }

        let sum = grid
            .into_iter()
            .filter_map(|((x, y), e)| {
                if e == &Entry::Symbol('*') {
                    Some((x, y))
                } else {
                    None
                }
            })
            .filter_map(|(x, y)| {
                let mut last_element = (isize::MAX, isize::MAX);
                let mut counter = 0;
                let mut product = 1;
                for (start, value) in grid
                    .neighbors8(x, y)
                    .filter_map(|((x, y), _)| find_number(grid, x, y))
                {
                    if last_element != start {
                        last_element = start;
                        product *= value;
                        counter += 1;
                    }
                }
                if counter == 2 {
                    Some(product)
                } else {
                    None
                }
            })
            .sum::<u32>();

        format!("{:#?}", sum)
    }
}

mod parsing {
    use super::Entry;
    use crate::utils::Grid;

    pub fn parse(input: &str) -> Grid<Entry> {
        Grid::parse(input, |c| {
            if c.is_digit(10) {
                Entry::Digit(c.to_digit(10).unwrap())
            } else if c != '.' {
                Entry::Symbol(c)
            } else {
                Entry::None
            }
        })
    }
}

gen_test!(
    a,
    Day03,
    r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    "4361"
);
gen_test!(
    b,
    Day03,
    r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    "467835"
);
