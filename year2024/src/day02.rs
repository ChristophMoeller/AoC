use super::*;

pub struct Day02;
impl Solution for Day02 {
    type Input<'a> = Vec<Vec<i64>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let count = input
            .iter()
            .filter(|x| {
                is_safe(&x, true, std::cmp::Ordering::Less)
                    || is_safe(&x, true, std::cmp::Ordering::Greater)
            })
            .count();

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let count = input
            .iter()
            .filter(|x| {
                is_safe(&x, false, std::cmp::Ordering::Less)
                    || is_safe(&x, false, std::cmp::Ordering::Greater)
                    || is_safe(&x[1..], true, std::cmp::Ordering::Less)
                    || is_safe(&x[1..], true, std::cmp::Ordering::Greater)
            })
            .count();

        format!("{}", count)
    }
}

fn is_safe(xs: &[i64], removed: bool, ord: std::cmp::Ordering) -> bool {
    match xs {
        [x, y, z, ..] => {
            (x.cmp(y) == ord && (x - y).abs() <= 3 && is_safe(&xs[1..], removed, ord))
                || (!removed
                    && x.cmp(z) == ord
                    && (x - z).abs() <= 3
                    && is_safe(&xs[2..], true, ord))
        }
        [x, y] => !removed || (x.cmp(y) == ord && (x - y).abs() <= 3),
        _ => true,
    }
}

mod parsing {}

gen_test!(
    a,
    Day02,
    r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    "2"
);

gen_test!(
    b,
    Day02,
    r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    "4"
);
