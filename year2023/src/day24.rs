use super::*;

use num_bigint::ToBigInt;
use num_rational::BigRational;

#[derive(Debug)]
pub struct Hailstone {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
}

impl Hailstone {
    /// Return the solution (s,t,d) to x + (s/d) delta_x = y + (t/d) delta_y.
    /// If d = 0, then there is no solution.
    fn intersection_xy(&self, other: &Self) -> (i64, i64, i64) {
        let rhs = [other.p.0 - self.p.0, other.p.1 - self.p.1];
        let lhs = [[self.v.0, -other.v.0], [self.v.1, -other.v.1]];
        let d = lhs[0][0] * lhs[1][1] - lhs[0][1] * lhs[1][0];

        let adj = [[lhs[1][1], -lhs[0][1]], [-lhs[1][0], lhs[0][0]]];

        let s = adj[0][0] * rhs[0] + adj[0][1] * rhs[1];
        let t = adj[1][0] * rhs[0] + adj[1][1] * rhs[1];
        (s, t, d)
    }
}

// I regret not using julia ...
fn find_ray(
    stones: &[Hailstone],
) -> (
    (BigRational, BigRational, BigRational),
    (BigRational, BigRational, BigRational),
) {
    let mut matrix = nalgebra::base::DMatrix::zeros(stones.len() * 3, 9);
    let mut rhs = nalgebra::base::DMatrix::zeros(stones.len() * 3, 1);

    for (i, s) in stones.iter().enumerate() {
        matrix[(3 * i + 0, 0)] = 1 as i128;
        matrix[(3 * i + 1, 1)] = 1 as i128;
        matrix[(3 * i + 2, 2)] = 1 as i128;

        matrix[(3 * i + 0, 4)] = s.v.2 as i128;
        matrix[(3 * i + 0, 5)] = -s.v.1 as i128;
        matrix[(3 * i + 0, 7)] = -s.p.2 as i128;
        matrix[(3 * i + 0, 8)] = s.p.1 as i128;

        matrix[(3 * i + 1, 3)] = -s.v.2 as i128;
        matrix[(3 * i + 1, 5)] = s.v.0 as i128;
        matrix[(3 * i + 1, 6)] = s.p.2 as i128;
        matrix[(3 * i + 1, 8)] = -s.p.0 as i128;

        matrix[(3 * i + 2, 3)] = s.v.1 as i128;
        matrix[(3 * i + 2, 4)] = -s.v.0 as i128;
        matrix[(3 * i + 2, 6)] = -s.p.1 as i128;
        matrix[(3 * i + 2, 7)] = s.p.0 as i128;

        rhs[(3 * i + 0, 0)] = (s.p.1 * s.v.2 - s.p.2 * s.v.1) as i128;
        rhs[(3 * i + 1, 0)] = (s.p.2 * s.v.0 - s.p.0 * s.v.2) as i128;
        rhs[(3 * i + 2, 0)] = (s.p.0 * s.v.1 - s.p.1 * s.v.0) as i128;
    }

    let mut rhs = (matrix.transpose() * rhs)
        .map(|x| BigRational::new(x.to_bigint().unwrap(), 1.to_bigint().unwrap()));
    let mut matrix = (matrix.transpose() * matrix)
        .map(|x| BigRational::new(x.to_bigint().unwrap(), 1.to_bigint().unwrap()));

    fn gauss_elim(
        m: &mut nalgebra::base::DMatrix<BigRational>,
        b: &mut nalgebra::base::DMatrix<BigRational>,
        k: usize,
        n: usize,
    ) {
        if k == n {
            return;
        }
        for i in (k + 1)..n {
            let factor = m[(i, k)].clone() / m[(k, k)].clone();
            for j in k..n {
                let a = m[(k, j)].clone();
                m[(i, j)] -= factor.clone() * a;
            }
            let a = b[(k, 0)].clone();
            b[(i, 0)] -= factor.clone() * a;
        }

        gauss_elim(m, b, k + 1, n);

        for i in (k + 1)..n {
            let a = b[(i, 0)].clone();
            b[(k, 0)] -= m[(k, i)].clone() * a;
            m[(k, i)] = 0.to_bigint().unwrap().into();
        }

        b[(k, 0)] /= m[(k, k)].clone();
        m[(k, k)] = 1.to_bigint().unwrap().into();
    }

    gauss_elim(&mut matrix, &mut rhs, 0, 9);

    (
        (
            rhs[(3, 0)].clone(),
            rhs[(4, 0)].clone(),
            rhs[(5, 0)].clone(),
        ),
        (
            rhs[(6, 0)].clone(),
            rhs[(7, 0)].clone(),
            rhs[(8, 0)].clone(),
        ),
    )
}

pub struct Day24;
impl Solution for Day24 {
    type Input<'a> = Vec<Hailstone>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        const X_RANGE: std::ops::RangeInclusive<i64> = 200000000000000..=400000000000000;
        const Y_RANGE: std::ops::RangeInclusive<i64> = 200000000000000..=400000000000000;

        let mut count = 0;

        for (i, x) in input.iter().enumerate() {
            for y in &input[(i + 1)..] {
                let (s, t, d) = x.intersection_xy(y);

                if d == 0 || d < 0 && (s > 0 || t > 0) || d > 0 && (s < 0 || t < 0) {
                    continue;
                }

                let x_start = i128::min(
                    d as i128 * (X_RANGE.start() - x.p.0) as i128,
                    d as i128 * (X_RANGE.end() - x.p.0) as i128,
                );
                let x_end = i128::max(
                    d as i128 * (X_RANGE.start() - x.p.0) as i128,
                    d as i128 * (X_RANGE.end() - x.p.0) as i128,
                );
                let y_start = i128::min(
                    d as i128 * (Y_RANGE.start() - x.p.1) as i128,
                    d as i128 * (Y_RANGE.end() - x.p.1) as i128,
                );
                let y_end = i128::max(
                    d as i128 * (Y_RANGE.start() - x.p.1) as i128,
                    d as i128 * (Y_RANGE.end() - x.p.1) as i128,
                );

                let x_range = x_start..x_end;
                let y_range = y_start..y_end;

                if x_range.contains(&(s as i128 * x.v.0 as i128))
                    && y_range.contains(&(s as i128 * x.v.1 as i128))
                {
                    count += 1;
                }
            }
        }

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let ((x, y, z), _) = find_ray(&input);

        format!("{}", x + y + z)
    }
}

mod parsing {
    use super::Hailstone;
    use nom::{
        character::complete::{char, i64, line_ending, space0},
        multi::{separated_list0, separated_list1},
        sequence::{separated_pair, tuple},
        *,
    };

    fn coord(input: &str) -> IResult<&str, (i64, i64, i64)> {
        separated_list1(tuple((space0, char(','), space0)), i64)
            .map(|p| (p[0], p[1], p[2]))
            .parse(input)
    }

    fn hailstone(input: &str) -> IResult<&str, Hailstone> {
        separated_pair(coord, tuple((space0, char('@'), space0)), coord)
            .map(|(p, v)| Hailstone { p, v })
            .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Hailstone>> {
        separated_list0(line_ending, hailstone)(input)
    }
}

// gen_test!(
//     a,
//     Day24,
//     r"19, 13, 30 @ -2,  1, -2
// 18, 19, 22 @ -1, -1, -2
// 20, 25, 34 @ -2, -2, -4
// 12, 31, 28 @ -1, -2, -1
// 20, 19, 15 @  1, -5, -3
// ",
//     "2"
// );
gen_test!(
    b,
    Day24,
    r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
",
    "47"
);
