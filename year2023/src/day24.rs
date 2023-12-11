use super::*;

pub struct Day24;
impl Solution for Day24 {
    type Input<'a> = ();

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(_input: &Self::Input<'a>) -> String {
        format!("{}", 0)
    }

    fn part_b<'a>(_input: &Self::Input<'a>) -> String {
        format!("{}", 0)
    }
}

mod parsing {
    use nom::*;

    pub(super) fn parse(input: &str) -> IResult<&str, ()> {
        Ok((input, ()))
    }
}

// gen_test!(a, Day24, r"", "");
// gen_test!(b, Day24, r"", "");
