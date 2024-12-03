use itertools::Itertools;

use super::*;

pub enum Inst {
    Mul(i64, i64),
    Do,
    Dont,
}

pub struct Day03;
impl Solution for Day03 {
    type Input<'a> = Vec<Inst>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        use super::parser::prelude::*;

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Token {
            Mul,
            Num(i64),
            Do,
            Dont,
        }

        let tokenizer_digit = parser::conditional1_pos(
            |p, c| p < 3 && c.is_digit(10),
            |c| Token::Num(c.parse::<i64>().unwrap()),
        );

        let tokenizer_mul = parser::tag("mul(", Token::Mul)
            .then(tokenizer_digit.clone())
            .then(parser::tag(",", Token::Mul).ignore())
            .then(tokenizer_digit.clone())
            .then(parser::tag(")", Token::Mul).ignore());

        let tokenizer = tokenizer_mul
            .or(parser::tag("do()", Token::Do))
            .or(parser::tag("don't()", Token::Dont))
            .or(parser::any())
            .repeated();

        let (tokens, _) = tokenizer.process(content);

        tokens
            .iter()
            .tuple_windows()
            .filter_map(|c| match c {
                (Token::Mul, Token::Num(a), Token::Num(b)) => Some(Inst::Mul(*a, *b)),
                (Token::Do, _, _) => Some(Inst::Do),
                (Token::Dont, _, _) => Some(Inst::Dont),
                _ => None,
            })
            .collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let sum: i64 = input
            .iter()
            .filter_map(|x| match x {
                Inst::Mul(a, b) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| a * b)
            .sum();
        format!("{sum}")
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut sum = 0;
        let mut enabled = true;
        for inst in input {
            match inst {
                Inst::Mul(a, b) => sum += a * b * if enabled { 1 } else { 0 },
                Inst::Do => enabled = true,
                Inst::Dont => enabled = false,
            }
        }
        format!("{sum}")
    }
}

gen_test!(
    a,
    Day03,
    r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    "161"
);

gen_test!(
    b,
    Day03,
    r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    "48"
);
