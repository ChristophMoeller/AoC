use super::*;

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
    type Input<'a> = Vec<Vec<Entry>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut current = 0;
        let mut next_to_symbol = false;
        let mut sum = 0;

        fn is_symbol(input: &Vec<Vec<Entry>>, r: isize, c: isize) -> bool {
            if r < 0 || c < 0 || r as usize >= input.len() || c as usize >= input[r as usize].len()
            {
                return false;
            }
            let r = r as usize;
            let c = c as usize;
            input[r][c].is_symbol()
        }

        for (r, row) in input.iter().enumerate() {
            for (c, &entry) in row.iter().enumerate() {
                let r = r as isize;
                let c = c as isize;
                match entry {
                    Entry::Digit(d) => {
                        current = current * 10 + d;
                        if is_symbol(input, r - 1, c - 1)
                            || is_symbol(input, r - 1, c)
                            || is_symbol(input, r - 1, c + 1)
                            || is_symbol(input, r, c - 1)
                            || is_symbol(input, r, c)
                            || is_symbol(input, r, c + 1)
                            || is_symbol(input, r + 1, c - 1)
                            || is_symbol(input, r + 1, c)
                            || is_symbol(input, r + 1, c + 1)
                        {
                            next_to_symbol = true;
                        }
                    }
                    _ => {
                        if next_to_symbol {
                            sum += current;
                        }
                        current = 0;
                        next_to_symbol = false;
                    }
                }
            }
            if next_to_symbol {
                sum += current;
            }
            current = 0;
            next_to_symbol = false;
        }

        format!("{:#?}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        fn find_number(
            input: &Vec<Vec<Entry>>,
            r: isize,
            c: isize,
        ) -> Option<((usize, usize), u32)> {
            if r < 0 || c < 0 || r as usize >= input.len() || c as usize >= input[r as usize].len()
            {
                return None;
            }
            let r = r as usize;
            let mut c = c as usize;

            if !input[r][c].is_digit() {
                return None;
            }

            while c > 0 && input[r][c - 1].is_digit() {
                c -= 1;
            }

            let start = (r, c);
            let mut current = 0;
            while let Entry::Digit(d) = input[r][c] {
                current = current * 10 + d;
                c += 1;
                if c >= input[r].len() {
                    break;
                }
            }

            return Some((start, current));
        }

        let sum = input
            .iter()
            .enumerate()
            .flat_map(|(r, row)| std::iter::repeat(r).zip(row.iter().enumerate()))
            .filter_map(|(r, (c, &e))| {
                if e == Entry::Symbol('*') {
                    Some((r, c))
                } else {
                    None
                }
            })
            .filter_map(|(r, c)| {
                let r = r as isize;
                let c = c as isize;
                let neighbors = [
                    (r - 1, c - 1),
                    (r - 1, c),
                    (r - 1, c + 1),
                    (r, c - 1),
                    (r, c + 1),
                    (r + 1, c - 1),
                    (r + 1, c),
                    (r + 1, c + 1),
                ];
                let mut last_element = (usize::MAX, usize::MAX);
                let mut counter = 0;
                let mut product = 1;
                for (start, value) in neighbors
                    .iter()
                    .filter_map(|(r, c)| find_number(input, *r, *c))
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

    pub fn parse(input: &str) -> Vec<Vec<Entry>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c.is_digit(10) {
                            Entry::Digit(c.to_digit(10).unwrap())
                        } else if c != '.' {
                            Entry::Symbol(c)
                        } else {
                            Entry::None
                        }
                    })
                    .collect()
            })
            .collect()
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
