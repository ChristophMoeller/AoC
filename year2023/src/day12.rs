use std::collections::HashMap;

use super::*;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

fn calc_possibilities_internal(
    status: &mut [&[Status]],
    amount: &mut [u32],
    cache: &mut HashMap<(u32, u32, u32, u32), u64>,
) -> u64 {
    if status.len() == 0 {
        return (amount.len() == 0 || amount.len() == 1 && amount[0] == 0) as u64;
    }

    if amount.len() == 0 {
        return status
            .iter()
            .all(|x| x.iter().all(|x| *x == Status::Unknown)) as u64;
    }

    let k = (
        status.len() as u32,
        status[0].len() as u32,
        amount.len() as u32,
        amount[0],
    );

    if let Some(x) = cache.get(&k) {
        *x
    } else {
        let x = if amount[0] as usize > status[0].len() {
            if status[0].iter().all(|x| *x == Status::Unknown) {
                calc_possibilities_internal(&mut status[1..], amount, cache)
            } else {
                0
            }
        } else if amount[0] as usize == status[0].len() {
            if status[0].iter().all(|x| *x == Status::Unknown) {
                calc_possibilities_internal(&mut status[1..], amount, cache)
                    + calc_possibilities_internal(&mut status[1..], &mut amount[1..], cache)
            } else {
                calc_possibilities_internal(&mut status[1..], &mut amount[1..], cache)
            }
        } else if status[0][0] == Status::Damaged {
            if status[0][amount[0] as usize] == Status::Unknown {
                let m = status[0];
                status[0] = &status[0][(amount[0] as usize + 1)..];
                let res = calc_possibilities_internal(status, &mut amount[1..], cache);
                status[0] = m;
                res
            } else {
                0
            }
        } else if status[0][0] == Status::Unknown {
            let m = status[0];
            status[0] = &status[0][1..];
            let mut res = calc_possibilities_internal(status, amount, cache);
            status[0] = m;
            if status[0][amount[0] as usize] == Status::Unknown {
                let m = status[0];
                status[0] = &status[0][(amount[0] as usize + 1)..];
                res += calc_possibilities_internal(status, &mut amount[1..], cache);
                status[0] = m;
            }
            res
        } else {
            unreachable!()
        };
        cache.insert(k, x);
        x
    }
}

fn calc_possibilities(status: &[Status], amount: &mut [u32]) -> u64 {
    let mut grouped_status = Vec::new();
    let mut remaining_status = status;
    while remaining_status.len() > 0 {
        let Some(p) = remaining_status
            .iter()
            .position(|x| *x == Status::Operational)
        else {
            grouped_status.push(remaining_status);
            break;
        };
        if p == 0 {
            remaining_status = &remaining_status[1..];
            continue;
        }

        grouped_status.push(&remaining_status[..p]);
        remaining_status = &remaining_status[(p + 1)..]
    }

    let mut cache = HashMap::new();

    calc_possibilities_internal(&mut grouped_status, amount, &mut cache)
}

pub struct Day12;
impl Solution for Day12 {
    type Input<'a> = Vec<(Vec<Status>, Vec<u32>)>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let sum = input
            .iter()
            .map(|(status, amount)| calc_possibilities(&status, &mut amount.clone()))
            .sum::<u64>();

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let sum = input
            .par_iter()
            .map(|(status, amount)| {
                let mut unfolded_status = Vec::new();
                unfolded_status.extend_from_slice(&status);
                unfolded_status.push(Status::Unknown);
                unfolded_status.extend_from_slice(&status);
                unfolded_status.push(Status::Unknown);
                unfolded_status.extend_from_slice(&status);
                unfolded_status.push(Status::Unknown);
                unfolded_status.extend_from_slice(&status);
                unfolded_status.push(Status::Unknown);
                unfolded_status.extend_from_slice(&status);
                let mut amount = amount
                    .iter()
                    .copied()
                    .cycle()
                    .take(amount.len() * 5)
                    .collect::<Vec<_>>();
                calc_possibilities(&unfolded_status, &mut amount)
            })
            .sum::<u64>();

        format!("{}", sum)
    }
}

mod parsing {
    use super::Status;
    use nom::{
        character::complete::char,
        character::complete::{line_ending, space1, u32},
        multi::{many1, separated_list1},
        sequence::separated_pair,
        *,
    };

    fn status(input: &str) -> IResult<&str, Vec<Status>> {
        many1(
            char('#')
                .map(|_| Status::Damaged)
                .or(char('.').map(|_| Status::Operational))
                .or(char('?').map(|_| Status::Unknown)),
        )
        .parse(input)
    }

    fn amount(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(char(','), u32)(input)
    }

    fn line(input: &str) -> IResult<&str, (Vec<Status>, Vec<u32>)> {
        separated_pair(status, space1, amount)(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<(Vec<Status>, Vec<u32>)>> {
        separated_list1(line_ending, line)(input)
    }
}

gen_test!(
    a,
    Day12,
    r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    "21"
);
gen_test!(
    b,
    Day12,
    r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    "525152"
);
