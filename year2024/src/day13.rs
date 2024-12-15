use super::*;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ClawSetup {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

pub struct Day13;
impl Solution for Day13 {
    type Input<'a> = Vec<ClawSetup>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let re = Regex::new(
            r"Button A: X\+([0-9]+), Y\+([0-9]+)\s+Button B: X\+([0-9]+), Y\+([0-9]+)\s+Prize: X=([0-9]+), Y=([0-9]+)"
        ).unwrap();

        re.captures_iter(content)
            .map(|c| c.extract())
            .map(|(_, [ax, ay, bx, by, px, py])| ClawSetup {
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (px.parse().unwrap(), py.parse().unwrap()),
            })
            .collect_vec()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let total_cost = input
            .iter()
            .map(|setup| {
                let m = [[setup.a.0, setup.b.0], [setup.a.1, setup.b.1]];
                let u = [setup.prize.0, setup.prize.1];
                let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
                let mt = [[m[1][1], -m[0][1]], [-m[1][0], m[0][0]]];
                let mtu = [
                    mt[0][0] * u[0] + mt[0][1] * u[1],
                    mt[1][0] * u[0] + mt[1][1] * u[1],
                ];
                if mtu[0] % det != 0 || mtu[1] % det != 0 {
                    return 0;
                }
                let sol = [mtu[0] / det, mtu[1] / det];
                if sol[0] <= 0 || sol[1] <= 0 {
                    return 0;
                }
                return sol[0] * 3 + sol[1];
            })
            .sum::<i64>();

        format!("{}", total_cost)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let total_cost = input
            .iter()
            .map(|setup| {
                let m = [[setup.a.0, setup.b.0], [setup.a.1, setup.b.1]];
                let u = [
                    setup.prize.0 + 10000000000000,
                    setup.prize.1 + 10000000000000,
                ];
                let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
                let mt = [[m[1][1], -m[0][1]], [-m[1][0], m[0][0]]];
                let mtu = [
                    mt[0][0] * u[0] + mt[0][1] * u[1],
                    mt[1][0] * u[0] + mt[1][1] * u[1],
                ];
                if mtu[0] % det != 0 || mtu[1] % det != 0 {
                    return 0;
                }
                let sol = [mtu[0] / det, mtu[1] / det];
                if sol[0] <= 0 || sol[1] <= 0 {
                    return 0;
                }
                return sol[0] * 3 + sol[1];
            })
            .sum::<i64>();

        format!("{}", total_cost)
    }
}

gen_test!(
    a,
    Day13,
    r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    "480"
);
