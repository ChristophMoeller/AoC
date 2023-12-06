use super::*;
use num_integer::Roots;

// This whole "parse once, use twice"-idea falls apart today

fn possibilities_to_win(time: u64, distance: u64) -> u64 {
    let rt = (time * time - 4 * (distance + 1)).sqrt();
    let a = (u64::MAX - 1) / 2 - ((u64::MAX - 1) - (time - rt)) / 2;
    let b = (time + rt) / 2;
    b - a + 1
}

pub struct Day06;
impl Solution for Day06 {
    type Input<'a> = Vec<(&'a str, &'a str)>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let p = input
            .iter()
            .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
            .map(|(time, distance)| possibilities_to_win(time, distance))
            .product::<u64>();

        format!("{}", p)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let (time, distance) = input.iter().fold(
            (String::new(), String::new()),
            |(mut acc_x, mut acc_y), (x, y)| {
                acc_x.push_str(x);
                acc_y.push_str(y);
                (acc_x, acc_y)
            },
        );
        let time = time.parse().unwrap();
        let distance = distance.parse().unwrap();

        let p = possibilities_to_win(time, distance);
        format!("{}", p)
    }
}

mod parsing {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending, space1},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        *,
    };

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_pair(
            tuple((tag("Time:"), space1, separated_list1(space1, digit1))),
            line_ending,
            tuple((tag("Distance:"), space1, separated_list1(space1, digit1))),
        )
        .map(|(a, b)| a.2.into_iter().zip(b.2.into_iter()).collect())
        .parse(input)
    }
}

gen_test!(
    a,
    Day06,
    r"Time:      7  15   30
Distance:  9  40  200",
    "288"
);
gen_test!(
    b,
    Day06,
    r"Time:      7  15   30
Distance:  9  40  200",
    "71503"
);
