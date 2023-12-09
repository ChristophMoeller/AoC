use super::*;

fn extrapolate_right(values: &mut [i64]) -> i64 {
    if values.len() == 0 {
        return 0;
    }

    for i in 0..(values.len() - 1) {
        values[i] = values[i + 1] - values[i];
    }

    let len = values.len() - 1;
    let d = extrapolate_right(&mut values[..len]);
    *values.last().unwrap() + d
}

fn extrapolate_left(values: &mut [i64]) -> i64 {
    if values.len() == 0 {
        return 0;
    }

    for i in (1..values.len()).rev() {
        values[i] = values[i] - values[i - 1];
    }

    let d = extrapolate_left(&mut values[1..]);
    *values.first().unwrap() - d
}

pub struct Day09;
impl Solution for Day09 {
    type Input<'a> = Vec<Vec<i64>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let sum: i64 = input
            .iter()
            .map(|line| extrapolate_right(&mut line.clone()))
            .sum();
        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let sum: i64 = input
            .iter()
            .map(|line| extrapolate_left(&mut line.clone()))
            .sum();
        format!("{}", sum)
    }
}

mod parsing {
    pub(super) fn parse(input: &str) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

gen_test!(
    a,
    Day09,
    r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    "114"
);
gen_test!(
    b,
    Day09,
    r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    "2"
);
