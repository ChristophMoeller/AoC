use std::collections::VecDeque;

use super::*;
use num_integer::Integer;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, Default)]
enum ModuleType<'a> {
    #[default]
    Broadcast,
    Conjunction(&'a str),
    FlipFlop(&'a str),
}

#[derive(Debug, Clone, Default)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    in_mask: u64,
    out_mask: u64,
    in_list: SmallVec<[u8; 64]>,
    out_list: SmallVec<[u8; 64]>,
}

#[derive(Debug)]
pub struct System<'a> {
    modules: Vec<Module<'a>>,
    broadcast: SmallVec<[u8; 64]>,
    rx: u8,
}

fn perform_step(
    system: &System,
    state: &mut u64,
) -> (u64, u64, SmallVec<[u64; 64]>, SmallVec<[u64; 64]>) {
    let mut queue = VecDeque::new();
    queue.extend(system.broadcast.iter().map(|x| (*x, false)));

    let mut low_count = 1;
    let mut high_count = 0;
    let mut detailed_low_count = SmallVec::new();
    detailed_low_count.extend(std::iter::repeat(0).take(system.modules.len()));
    let mut detailed_high_count = SmallVec::new();
    detailed_high_count.extend(std::iter::repeat(0).take(system.modules.len()));

    while let Some((idx, high)) = queue.pop_front() {
        low_count += !high as u64;
        high_count += high as u64;

        detailed_low_count[idx as usize] += !high as u64;
        detailed_high_count[idx as usize] += high as u64;

        match system.modules[idx as usize].module_type {
            ModuleType::FlipFlop(_) => {
                if !high {
                    *state ^= 1 << idx;
                    queue.extend(
                        system.modules[idx as usize]
                            .out_list
                            .iter()
                            .map(|x| (*x, *state & (1 << idx) == 1 << idx)),
                    )
                }
            }
            ModuleType::Conjunction(_) => {
                if *state & system.modules[idx as usize].in_mask
                    == system.modules[idx as usize].in_mask
                {
                    *state |= 1 << idx;
                } else {
                    *state &= u64::MAX ^ (1 << idx);
                }
                queue.extend(
                    system.modules[idx as usize]
                        .out_list
                        .iter()
                        .map(|x| (*x, *state & (1 << idx) != 1 << idx)),
                )
            }
            ModuleType::Broadcast => {}
        }
    }

    (
        low_count,
        high_count,
        detailed_low_count,
        detailed_high_count,
    )
}

pub struct Day20;
impl Solution for Day20 {
    type Input<'a> = System<'a>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut state = 0;

        let mut low_count = 0;
        let mut high_count = 0;

        for _ in 0..1000 {
            let (low, high, _, _) = perform_step(input, &mut state);
            low_count += low;
            high_count += high;
        }

        format!("{}", low_count * high_count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let s0 = input.modules[input.rx as usize].in_list[0];
        let res = input.modules[s0 as usize]
            .in_list
            .iter()
            .map(|s| {
                let mut state = 0;
                let mut counter = 1u64;
                while perform_step(input, &mut state).2[*s as usize] == 0 {
                    counter += 1;
                }
                counter
            })
            .fold(1u64, |acc, x| acc.lcm(&x));

        format!("{}", res)
    }
}

mod parsing {
    use std::collections::HashMap;

    use super::ModuleType;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, line_ending, space0},
        multi::separated_list0,
        sequence::{separated_pair, tuple},
        *,
    };
    use smallvec::SmallVec;

    fn module_name(input: &str) -> IResult<&str, ModuleType> {
        alt((
            tuple((char('%'), alpha1)).map(|(_, name)| ModuleType::FlipFlop(name)),
            tuple((char('&'), alpha1)).map(|(_, name)| ModuleType::Conjunction(name)),
            tag("broadcaster").map(|_| ModuleType::Broadcast),
        ))(input)
    }

    fn module_neighbors(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list0(tuple((space0, char(','), space0)), alpha1)(input)
    }

    fn module(input: &str) -> IResult<&str, (ModuleType, Vec<&str>)> {
        separated_pair(
            module_name,
            tuple((space0, tag("->"), space0)),
            module_neighbors,
        )(input)
    }

    fn module_list(input: &str) -> IResult<&str, Vec<(ModuleType, Vec<&str>)>> {
        separated_list0(line_ending, module)(input)
    }

    pub(super) fn parse(input: &str) -> super::System {
        let modules = module_list(input).unwrap().1;

        let mut broadcast: SmallVec<[u8; 64]> = SmallVec::new();
        let mut map = HashMap::new();

        let mut listed_modules = Vec::new();
        let mut counter = 0u8;

        for (module, neighbors) in modules {
            let mut out_list: SmallVec<[u8; 64]> = SmallVec::new();

            for neighbor in neighbors {
                let c = *map.entry(neighbor).or_insert_with(|| {
                    listed_modules.push(super::Module::default());
                    let c = counter;
                    counter += 1;
                    c
                });

                out_list.push(c);
            }

            let mut out_mask = 0u64;
            for out_neighbor in &out_list {
                out_mask |= 1 << out_neighbor;
            }

            match module {
                ModuleType::FlipFlop(name) | ModuleType::Conjunction(name) => {
                    let c = *map.entry(name).or_insert_with(|| {
                        listed_modules.push(super::Module::default());
                        let c = counter;
                        counter += 1;
                        c
                    });

                    for out_neighbor in &out_list {
                        listed_modules[*out_neighbor as usize].in_list.push(c);
                        listed_modules[*out_neighbor as usize].in_mask |= 1 << c;
                    }

                    listed_modules[c as usize].out_list = out_list;
                    listed_modules[c as usize].out_mask = out_mask;

                    listed_modules[c as usize].module_type = module;
                }
                ModuleType::Broadcast => {
                    broadcast = out_list;
                }
            }
        }

        super::System {
            modules: listed_modules,
            broadcast,
            rx: map["rx"],
        }
    }
}

gen_test!(
    a,
    Day20,
    r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con-> rx
",
    "11687500"
);
// gen_test!(b, Day20, r"", "");
