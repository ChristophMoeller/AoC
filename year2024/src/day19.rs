use itertools::Itertools;

use super::*;

pub struct Day19;
impl Solution for Day19 {
    type Input<'a> = (Vec<&'a str>, Vec<&'a str>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let mut lines = content.lines();
        let available = lines.next().unwrap().split(", ").collect_vec();

        let patterns = lines.skip(1).collect_vec();

        (available, patterns)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let (available, pattern) = input;

        let count = pattern
            .iter()
            .filter(|x| constructable(&available, x) > 0)
            .count();

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let (available, pattern) = input;

        let count: u64 = pattern.iter().map(|x| constructable(&available, x)).sum();

        format!("{}", count)
    }
}

fn constructable(available: &[&str], pattern: &str) -> u64 {
    let mut prior = vec![0u64; pattern.len() + 1];
    prior[0] = 1;

    for i in 1..=pattern.len() {
        for towel in available {
            if i >= towel.len() {
                if pattern[(i - towel.len())..]
                    .bytes()
                    .zip(towel.bytes())
                    .all(|(x, y)| x == y)
                {
                    prior[i] += prior[i - towel.len()];
                }
            }
        }
    }

    prior[pattern.len()]
}

gen_test!(
    a,
    Day19,
    r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    "6"
);

gen_test!(
    b,
    Day19,
    r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    "16"
);
