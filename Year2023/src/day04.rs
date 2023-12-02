use super::*;

pub struct Day04;
impl Solution for Day04 {
    type Input<'a> = &'a str;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
    }

    fn part_a<'a>(_input: &Self::Input<'a>) -> String {
        format!("{}", 0)
    }

    fn part_b<'a>(_input: &Self::Input<'a>) -> String {
        format!("{}", 0)
    }
}

mod parsing {}

// gen_test!(a, Day04, r"", "");
// gen_test!(b, Day04, r"", "");
