use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone)]
pub struct PageUpdates {
    ordering: HashSet<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

pub struct Day05;
impl Solution for Day05 {
    type Input<'a> = PageUpdates;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let lines = content.lines();
        let mut ordering = HashSet::new();

        for line in lines.take_while(|l| l.len() > 0) {
            let mut parts = line.split('|');
            ordering.insert((
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ));
        }

        let rest = content.lines().skip_while(|l| l.len() > 0).skip(1);

        let updates = rest
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();

        PageUpdates { ordering, updates }
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let res: u32 = input
            .updates
            .iter()
            .filter(|u| {
                for i in 0..u.len() {
                    for j in (i + 1)..u.len() {
                        let (x, y) = (u[j], u[i]);
                        if input.ordering.contains(&(x, y)) {
                            return false;
                        }
                    }
                }
                return true;
            })
            .map(|u| u[u.len() / 2])
            .sum();

        format!("{}", res)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let res: u32 = input
            .updates
            .iter()
            .filter_map(|u| {
                let mut update = u.clone();
                if order(&input, &mut update) {
                    return None;
                }
                Some(update)
            })
            .map(|u| u[u.len() / 2])
            .sum();

        format!("{}", res)
    }
}

fn order(input: &PageUpdates, update: &mut Vec<u32>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            let (x, y) = (update[j], update[i]);
            if input.ordering.contains(&(x, y)) {
                update.swap(i, j);
                order(input, update);
                return false;
            }
        }
    }
    return true;
}

gen_test!(
    a,
    Day05,
    r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    "143"
);

gen_test!(
    b,
    Day05,
    r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    "123"
);
