use std::{collections::HashMap, ops::RangeInclusive};

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Clone, Debug, Default)]
pub struct Part {
    extremely_cool_looking: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32,
}

#[derive(Clone, Copy, Debug)]
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
        match self.category {
            Category::ExtremelyCoolLooking => self.op.check(part.extremely_cool_looking, self.v),
            Category::Musical => self.op.check(part.musical, self.v),
            Category::Aerodynamic => self.op.check(part.aerodynamic, self.v),
            Category::Shiny => self.op.check(part.shiny, self.v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Workflow<'a> {
    tests: Vec<(Condition, &'a str)>,
    otherwise: &'a str,
}

impl<'a> Workflow<'a> {
    fn apply_to_part(&self, part: &Part) -> &'a str {
        for test in &self.tests {
            if test.0.check(part) {
                return test.1;
            }
        }

        self.otherwise
    }

    fn apply_to_intervals(
        &self,
        (mut x, mut m, mut a, mut s): (
            RangeInclusive<u32>,
            RangeInclusive<u32>,
            RangeInclusive<u32>,
            RangeInclusive<u32>,
        ),
    ) -> Vec<(
        (
            RangeInclusive<u32>,
            RangeInclusive<u32>,
            RangeInclusive<u32>,
            RangeInclusive<u32>,
        ),
        &'a str,
    )> {
        let mut res = Vec::new();

        for (con, w) in &self.tests {
            let lower_check = con.check(&Part {
                extremely_cool_looking: *x.start(),
                musical: *m.start(),
                aerodynamic: *a.start(),
                shiny: *s.start(),
            });
            let upper_check = con.check(&Part {
                extremely_cool_looking: *x.end(),
                musical: *m.end(),
                aerodynamic: *a.end(),
                shiny: *s.end(),
            });

            let (lower_offset, upper_offset) = match con.op {
                ConditionOperator::Less => (1, 0),
                ConditionOperator::Greater => (0, 1),
            };

            let (mut lower, mut upper) = match con.category {
                Category::ExtremelyCoolLooking => (
                    (
                        *x.start()..=(con.v - lower_offset),
                        m.clone(),
                        a.clone(),
                        s.clone(),
                    ),
                    (
                        (con.v + upper_offset)..=*x.end(),
                        m.clone(),
                        a.clone(),
                        s.clone(),
                    ),
                ),
                Category::Musical => (
                    (
                        x.clone(),
                        *m.start()..=(con.v - lower_offset),
                        a.clone(),
                        s.clone(),
                    ),
                    (
                        x.clone(),
                        (con.v + upper_offset)..=*m.end(),
                        a.clone(),
                        s.clone(),
                    ),
                ),
                Category::Aerodynamic => (
                    (
                        x.clone(),
                        m.clone(),
                        *a.start()..=(con.v - lower_offset),
                        s.clone(),
                    ),
                    (
                        x.clone(),
                        m.clone(),
                        (con.v + upper_offset)..=*a.end(),
                        s.clone(),
                    ),
                ),
                Category::Shiny => (
                    (
                        x.clone(),
                        m.clone(),
                        a.clone(),
                        *s.start()..=(con.v - lower_offset),
                    ),
                    (
                        x.clone(),
                        m.clone(),
                        a.clone(),
                        (con.v + upper_offset)..=*s.end(),
                    ),
                ),
            };

            lower.0 = *lower.0.start()..=*lower.0.end().min(x.end());
            lower.1 = *lower.1.start()..=*lower.1.end().min(m.end());
            lower.2 = *lower.2.start()..=*lower.2.end().min(a.end());
            lower.3 = *lower.3.start()..=*lower.3.end().min(s.end());
            upper.0 = *upper.0.start().max(x.start())..=*upper.0.end();
            upper.1 = *upper.1.start().max(x.start())..=*upper.1.end();
            upper.2 = *upper.2.start().max(x.start())..=*upper.2.end();
            upper.3 = *upper.3.start().max(x.start())..=*upper.3.end();

            match (lower_check, upper_check) {
                (true, true) => {
                    res.push((lower, *w));
                    break;
                }
                (true, false) => {
                    res.push((lower, *w));
                    (x, m, a, s) = upper;
                }
                (false, true) => {
                    res.push((upper, *w));
                    (x, m, a, s) = lower;
                }
                (false, false) => {}
            }
        }

        res.push(((x, m, a, s), self.otherwise));

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
            .map(|part| {
                (part.extremely_cool_looking + part.musical + part.aerodynamic + part.shiny) as u64
            })
            .sum::<u64>();

        format!("{}", res)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut partition = vec![((1..=4000, 1..=4000, 1..=4000, 1..=4000), "in")];

        let mut accepted = 0u64;

        while let Some((i, w)) = partition.pop() {
            for (i, w) in input.0[w].apply_to_intervals(i) {
                if w == "A" {
                    accepted += (i.0.count() * i.1.count() * i.2.count() * i.3.count()) as u64;
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
                match c {
                    Category::ExtremelyCoolLooking => acc.extremely_cool_looking = v,
                    Category::Musical => acc.musical = v,
                    Category::Aerodynamic => acc.aerodynamic = v,
                    Category::Shiny => acc.shiny = v,
                };
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
