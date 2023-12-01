use super::*;

pub struct Day01;
impl Solution for Day01 {
    type Input<'a> = Vec<&'a str>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content.lines().collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let line_parser = parsing::first_and_last(parsing::explicit_digit);

        let sum: u32 = input
            .iter()
            .copied()
            .filter_map(line_parser)
            .map(|(a, b)| 10 * a + b)
            .sum();
        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let line_parser = parsing::first_and_last(parsing::digit);

        let sum: u32 = input
            .iter()
            .copied()
            .filter_map(line_parser)
            .map(|(a, b)| 10 * a + b)
            .sum();
        format!("{}", sum)
    }
}

mod parsing {
    // Using nom is a bit of an overkill for this problem.
    // ... but it's good practice, i guess.
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map_res,
        error::ParseError,
        Parser, *,
    };

    pub fn explicit_digit(input: &str) -> IResult<&str, u32> {
        map_res(take(1usize), |s: &str| s.parse::<u32>())(input)
    }

    pub fn named_digit(input: &str) -> IResult<&str, u32> {
        alt((
            tag("one").map(|_| 1),
            tag("two").map(|_| 2),
            tag("three").map(|_| 3),
            tag("four").map(|_| 4),
            tag("five").map(|_| 5),
            tag("six").map(|_| 6),
            tag("seven").map(|_| 7),
            tag("eight").map(|_| 8),
            tag("nine").map(|_| 9),
        ))(input)
    }

    pub fn digit(input: &str) -> IResult<&str, u32> {
        alt((explicit_digit, named_digit))(input)
    }

    pub fn first_and_last<'a, O, E: ParseError<&'a str>, P: Parser<&'a str, O, E>>(
        mut parser: P,
    ) -> impl FnMut(&'a str) -> Option<(O, O)> {
        move |input: &str| {
            let mut try_parse = |(i, _)| parser.parse(&input[i..]).ok().map(|(_, o)| o);

            let first = input.char_indices().find_map(&mut try_parse)?;
            let last = input.char_indices().rev().find_map(&mut try_parse)?;

            Some((first, last))
        }
    }
}

gen_test!(
    a,
    Day01,
    r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    "142"
);

gen_test!(
    b,
    Day01,
    r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    "281"
);
