use itertools::Itertools;

use super::*;

#[derive(Debug, Clone)]
pub struct Map<'a> {
    _from: &'a str,
    _to: &'a str,
    ranges: Vec<(std::ops::Range<u64>, std::ops::Range<u64>)>,
}

impl<'a> Map<'a> {
    fn apply(&self, x: u64) -> u64 {
        for (dest, src) in &self.ranges {
            if src.contains(&x) {
                return dest.start + (x - src.start);
            }
        }
        x
    }
    fn apply_range(&self, x: std::ops::Range<u64>) -> Vec<std::ops::Range<u64>> {
        let mut res = Vec::new();
        let mut cut_input = Vec::new();
        for (dest, src) in &self.ranges {
            let left = x.start.max(src.start);
            let right = x.end.min(src.end);
            if left < right {
                let offset = left - src.start;
                let length = right - left;
                let start = dest.start + offset;
                res.push(start..(start + length));
                cut_input.push(left..right);
            }
        }

        cut_input.sort_by(|a, b| a.start.cmp(&b.start));
        cut_input.push(u64::MAX..u64::MAX);

        let mut last_x = x.start;
        for cut in cut_input {
            if last_x >= x.end {
                break;
            }
            if cut.start >= x.end {
                res.push(last_x..x.end);
                break;
            }
            if last_x < cut.start {
                res.push(last_x..cut.start);
            }
            last_x = cut.end;
        }

        res
    }
}

pub struct Day05;
impl Solution for Day05 {
    type Input<'a> = (Vec<u64>, Vec<Map<'a>>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let closest = input
            .0
            .iter()
            .copied()
            .map(|mut x| {
                for m in &input.1 {
                    x = m.apply(x);
                }
                x
            })
            .min()
            .unwrap();

        format!("{}", closest)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let xs = input
            .0
            .iter()
            .tuples()
            .map(|(&start, &length)| start..(start + length))
            .collect_vec();

        let closest = input
            .1
            .iter()
            .fold(xs, |acc, map| {
                acc.iter()
                    .cloned()
                    .map(|x| map.apply_range(x))
                    .flatten()
                    .collect_vec()
            })
            .iter()
            .map(|x| x.start)
            .min()
            .unwrap();

        format!("{}", closest)
    }
}

mod parsing {
    use super::Map;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, space1, u64},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        *,
    };

    fn parse_range(input: &str) -> IResult<&str, (std::ops::Range<u64>, std::ops::Range<u64>)> {
        tuple((u64, space1, u64, space1, u64))
            .map(|(a, _, b, _, c)| (a..(a + c), b..(b + c)))
            .parse(input)
    }

    fn parse_map(input: &str) -> IResult<&str, Map> {
        separated_pair(
            tuple((alpha1, tag("-to-"), alpha1, tag(" map:"))).map(|(from, _, to, _)| (from, to)),
            line_ending,
            separated_list1(line_ending, parse_range),
        )
        .map(|((from, to), ranges)| Map {
            _from: from,
            _to: to,
            ranges,
        })
        .parse(input)
    }

    fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        tuple((tag("seeds: "), separated_list1(space1, u64)))
            .map(|(_, seeds)| seeds)
            .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
        separated_pair(
            parse_seeds,
            tuple((line_ending, line_ending)),
            separated_list1(tuple((line_ending, line_ending)), parse_map),
        )(input)
    }
}

gen_test!(
    a,
    Day05,
    r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    "35"
);
gen_test!(
    b,
    Day05,
    r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    "46"
);
