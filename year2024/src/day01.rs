use super::*;
use std::collections::HashMap;

// Writing Rust code again ...
// ... it's been a while.

pub struct Day01;
impl Solution for Day01 {
    type Input<'a> = (Vec<i64>, Vec<i64>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        // How the heck am I supposed to know that `upzip` automatically collects my iterators?
        content
            .lines()
            .map(|line| {
                let mut sp = line.split_whitespace();
                return (
                    sp.next().unwrap().parse::<i64>().unwrap(),
                    sp.next().unwrap().parse::<i64>().unwrap(),
                );
            })
            .unzip()
        // Took me way to long to figure it out....
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let (mut a, mut b) = input.clone();
        a.sort();
        b.sort();
        let sum: i64 = a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).sum();
        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let (a, b) = input;
        let mut b_map: HashMap<i64, i64> = HashMap::new();
        for b in b {
            b_map.entry(*b).and_modify(|x| *x += 1).or_insert(1);
        }
        let sum = a
            .iter()
            .map(|&x| *b_map.get(&x).unwrap_or(&0) * x)
            .sum::<i64>();
        format!("{}", sum)
    }
}

mod parsing {}

gen_test!(
    a,
    Day01,
    r"3   4
4   3
2   5
1   3
3   9
3   3",
    "11"
);

gen_test!(
    b,
    Day01,
    r"3   4
4   3
2   5
1   3
3   9
3   3",
    "31"
);
