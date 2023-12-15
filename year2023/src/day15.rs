use std::collections::VecDeque;

use super::*;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(u8),
    Remove,
}

fn hash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0u8, |acc, &c| acc.wrapping_add(c).wrapping_mul(17))
}

pub struct Day15;
impl Solution for Day15 {
    type Input<'a> = Vec<&'a str>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content.split(',').collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let h: u64 = input.iter().map(|instr| hash(instr) as u64).sum();
        format!("{}", h)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut boxes: [_; 256] = core::array::from_fn(|_| VecDeque::new());
        for (label, instr) in input.iter().map(|s| parsing::parse(s).unwrap().1) {
            let h = hash(label);
            match instr {
                Instruction::Set(focal_length) => {
                    if let Some(p) = boxes[h as usize].iter().position(|(l, _)| *l == label) {
                        boxes[h as usize][p].1 = focal_length;
                    } else {
                        boxes[h as usize].push_back((label, focal_length))
                    }
                }
                Instruction::Remove => {
                    if let Some(p) = boxes[h as usize].iter().position(|(l, _)| *l == label) {
                        boxes[h as usize].remove(p);
                    }
                }
            }
        }

        let res: usize = boxes
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                b.into_iter()
                    .map(|(_, f)| f)
                    .enumerate()
                    .map(|(j, f)| (i + 1) * (j + 1) * f as usize)
                    .sum::<usize>()
            })
            .sum();

        format!("{}", res)
    }
}

mod parsing {
    use super::Instruction;
    use nom::{
        character::complete::{alpha1, char, u8},
        sequence::tuple,
        *,
    };

    pub(super) fn parse<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Instruction)> {
        tuple((
            alpha1,
            char('-')
                .map(|_| Instruction::Remove)
                .or(tuple((char('='), u8)).map(|(_, f)| Instruction::Set(f))),
        ))(input)
    }
}

gen_test!(
    a,
    Day15,
    r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
    "1320"
);
gen_test!(
    b,
    Day15,
    r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
    "145"
);
