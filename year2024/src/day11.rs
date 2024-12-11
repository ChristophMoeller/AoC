use super::*;

use std::collections::HashMap;

pub struct Day11;
impl Solution for Day11 {
    type Input<'a> = Vec<u64>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut lookup = HashMap::new();
        let mut count = 0;
        for x in input {
            count += number_stones(*x, 25, &mut lookup);
        }

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut lookup = HashMap::new();
        let mut count = 0;
        for x in input {
            count += number_stones(*x, 75, &mut lookup);
        }

        format!("{}", count)
    }
}

fn number_stones(x: u64, r: u64, lookup: &mut HashMap<(u64, u64), u64>) -> u64 {
    if r == 0 {
        return 1;
    }
    if let Some(c) = lookup.get(&(r, x)) {
        return *c;
    }
    let l10 = log10(x);
    let v = if x == 0 {
        number_stones(1, r - 1, lookup)
    } else if l10 % 2 == 0 {
        let a = exp10(l10 / 2);
        number_stones(x / a, r - 1, lookup) + number_stones(x % a, r - 1, lookup)
    } else {
        number_stones(x * 2024, r - 1, lookup)
    };
    lookup.insert((r, x), v);
    v
}

fn exp10(x: u64) -> u64 {
    if x == 0 {
        return 1;
    }
    return 10 * exp10(x - 1);
}

fn log10(x: u64) -> u64 {
    if x < 10 {
        return 1;
    }
    return log10(x / 10) + 1;
}

gen_test!(a, Day11, r"125 17", "55312");
