use std::{collections::HashMap, ops::RangeInclusive};

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn index(&self) -> usize {
        *self as usize
    }
}

type Part = [u32; 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConditionOperator {
    Less,
    Greater,
}

impl ConditionOperator {
    #[inline(always)]
    fn check(&self, a: u32, b: u32) -> bool {
        match self {
            ConditionOperator::Less => a < b,
            ConditionOperator::Greater => a > b,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Condition {
    category: Category,
    op: ConditionOperator,
    v: u32,
}

impl Condition {
    #[inline(always)]
    fn check(&self, part: &Part) -> bool {
        self.op.check(part[self.category.index()], self.v)
    }
}

#[derive(Clone, Debug)]
pub struct Workflow<'a> {
    tests: Vec<(Condition, &'a str)>,
    otherwise: &'a str,
}

type Interval = [RangeInclusive<u32>; 4];

impl<'a> Workflow<'a> {
    fn apply_to_part(&self, part: &Part) -> &'a str {
        for test in &self.tests {
            if test.0.check(part) {
                return test.1;
            }
        }

        self.otherwise
    }

    fn apply_to_intervals(&self, mut int: Interval) -> Vec<(Interval, &'a str)> {
        let mut res = Vec::new();

        for (con, w) in &self.tests {
            if int[con.category.index()].contains(&con.v) {
                let mut lower = int.clone();
                let mut upper = int.clone();

                if con.op == ConditionOperator::Less {
                    lower[con.category.index()] =
                        *lower[con.category.index()].start()..=(con.v - 1);
                    upper[con.category.index()] = con.v..=*upper[con.category.index()].end();
                    res.push((lower, *w));
                    int = upper;
                } else if con.op == ConditionOperator::Greater {
                    lower[con.category.index()] = *lower[con.category.index()].start()..=con.v;
                    upper[con.category.index()] = (con.v + 1)..=*upper[con.category.index()].end();
                    res.push((upper, *w));
                    int = lower;
                }
            } else if con.op.check(*int[con.category.index()].start(), con.v) {
                res.push((int.clone(), *w));
                break;
            }
        }

        res.push((int, self.otherwise));

        res
    }
}

pub struct Day19;
impl Solution for Day19 {
    type Input<'a> = (HashMap<&'a str, Workflow<'a>>, Vec<Part>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let (workflows, parts) = parsing::parse(content).unwrap().1;

        let workflows: HashMap<_, _> = workflows.into_iter().collect();

        (workflows, parts)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let res = input
            .1
            .iter()
            .filter(|part| {
                let mut current_workflow = "in";
                while current_workflow != "R" && current_workflow != "A" {
                    current_workflow = input.0[current_workflow].apply_to_part(part);
                }

                current_workflow == "A"
            })
            .map(|part| part.iter().sum::<u32>() as u64)
            .sum::<u64>();

        format!("{}", res)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut partition = vec![([1..=4000, 1..=4000, 1..=4000, 1..=4000], "in")];

        let mut accepted = 0u64;

        while let Some((i, w)) = partition.pop() {
            for (i, w) in input.0[w].apply_to_intervals(i) {
                if w == "A" {
                    accepted += i.iter().map(|a| a.clone().count() as u64).product::<u64>() as u64;
                } else if w != "R" {
                    partition.push((i, w))
                }
            }

            println!("{:#?}", partition);
        }

        format!("{}", accepted)
    }
}

mod parsing {
    use crate::day19::Condition;
    use crate::day19::ConditionOperator;

    use super::Category;
    use super::Part;
    use super::Workflow;
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::line_ending;
    use nom::character::complete::{char, u32};
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::sequence::separated_pair;
    use nom::sequence::tuple;
    use nom::*;

    fn part(input: &str) -> IResult<&str, Part> {
        fn category_value(input: &str) -> IResult<&str, (Category, u32)> {
            separated_pair(
                alt((
                    char('x').map(|_| Category::ExtremelyCoolLooking),
                    char('m').map(|_| Category::Musical),
                    char('a').map(|_| Category::Aerodynamic),
                    char('s').map(|_| Category::Shiny),
                )),
                char('='),
                u32,
            )(input)
        }

        let (input, values) = delimited(
            char('{'),
            separated_list0(char(','), category_value),
            char('}'),
        )(input)?;

        let res = values
            .into_iter()
            .fold(Default::default(), |mut acc: Part, (c, v)| {
                acc[c.index()] = v;
                acc
            });

        Ok((input, res))
    }

    fn workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
        fn test(input: &str) -> IResult<&str, (Condition, &str)> {
            tuple((
                alt((
                    char('x').map(|_| Category::ExtremelyCoolLooking),
                    char('m').map(|_| Category::Musical),
                    char('a').map(|_| Category::Aerodynamic),
                    char('s').map(|_| Category::Shiny),
                )),
                alt((
                    char('<').map(|_| ConditionOperator::Less),
                    char('>').map(|_| ConditionOperator::Greater),
                )),
                u32,
                char(':'),
                alpha1,
            ))
            .map(|(cat, op, v, _, w)| {
                (
                    Condition {
                        category: cat,
                        op,
                        v,
                    },
                    w,
                )
            })
            .parse(input)
        }

        tuple((
            alpha1,
            delimited(
                char('{'),
                separated_pair(separated_list0(char(','), test), char(','), alpha1)
                    .map(|(tests, otherwise)| Workflow { tests, otherwise }),
                char('}'),
            ),
        ))(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, (Vec<(&str, Workflow)>, Vec<Part>)> {
        separated_pair(
            separated_list0(line_ending, workflow),
            tuple((line_ending, line_ending)),
            separated_list0(line_ending, part),
        )(input)
    }
}

gen_test!(
    a,
    Day19,
    r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    "19114"
);
gen_test!(
    b,
    Day19,
    r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    "167409079868000"
);
