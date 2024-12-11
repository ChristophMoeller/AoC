use itertools::Itertools;

use super::*;

pub struct Day09;
impl Solution for Day09 {
    type Input<'a> = Vec<(Option<u64>, u64)>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let mut id = 0;
        let ids = std::iter::from_fn(move || {
            id += 1;
            Some([Some(id - 1), None].into_iter())
        })
        .flatten();
        let counts = content
            .chars()
            .filter_map(|c| c.to_digit(10).map(|x| x as u64));
        ids.zip(counts).collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut input = input.clone();
        let mut l = 0;
        let mut r = input.len() - 1;
        let filled = std::iter::from_fn(|| {
            while r > l && input[r].0.is_none() {
                r -= 1;
            }
            if input[l].0.is_some() {
                l += 1;
                return Some(std::iter::repeat_n(
                    input[l - 1].0.unwrap(),
                    input[l - 1].1 as usize,
                ));
            }
            if l >= r {
                return None;
            }
            if input[l].1 > input[r].1 {
                input[l].1 -= input[r].1;
                r -= 1;
                return Some(std::iter::repeat_n(
                    input[r + 1].0.unwrap(),
                    input[r + 1].1 as usize,
                ));
            }
            if input[l].1 < input[r].1 {
                input[r].1 -= input[l].1;
                l += 1;
                return Some(std::iter::repeat_n(
                    input[r].0.unwrap(),
                    input[l - 1].1 as usize,
                ));
            }
            if input[l].1 == input[r].1 {
                l += 1;
                r -= 1;
                return Some(std::iter::repeat_n(
                    input[r + 1].0.unwrap(),
                    input[r + 1].1 as usize,
                ));
            }
            None
        })
        .flatten();

        format!(
            "{}",
            filled.enumerate().map(|(x, y)| x as u64 * y).sum::<u64>()
        )
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut input = input.clone();

        let mut r = input.len() - 1;
        while r > 0 {
            while input[r].0.is_none() {
                r -= 1;
            }
            if let Some((pos, _)) = input
                .iter()
                .find_position(|x| x.0.is_none() && x.1 >= input[r].1)
            {
                if pos > r {
                    r -= 1;
                    continue;
                }
                input.insert(pos, input[r]);
                input[pos + 1].1 -= input[r + 1].1;
                input[r + 1].0 = None;
            } else {
                r -= 1;
            }
        }

        let sum = input
            .iter()
            .map(|(t, c)| std::iter::repeat_n(t.unwrap_or(0), *c as usize))
            .flatten()
            .enumerate()
            .map(|(x, y)| x as u64 * y)
            .sum::<u64>();

        format!("{}", sum)
    }
}

gen_test!(a, Day09, r"2333133121414131402", "1928");
gen_test!(b, Day09, r"2333133121414131402", "2858");
