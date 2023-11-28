use super::*;

pub struct Day01;

impl Solution for Day01 {
    type Input<'a> = &'a str;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        input.to_string()
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        input.to_string()
    }
}

gen_test!(a, Day01, "x", "x");
gen_test!(b, Day01, "x", "x");
