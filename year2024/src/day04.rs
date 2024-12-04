use super::*;

pub struct Day04;
impl Solution for Day04 {
    type Input<'a> = Vec<Vec<char>>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        content.lines().map(|line| line.chars().collect()).collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mask1 = [[Some('X'), Some('M'), Some('A'), Some('S')]];
        let mask2 = rotate_mask(&mask1);
        let mask3 = rotate_mask(&mask2);
        let mask4 = rotate_mask(&mask3);

        let dmask1 = [
            [Some('X'), None, None, None],
            [None, Some('M'), None, None],
            [None, None, Some('A'), None],
            [None, None, None, Some('S')],
        ];
        let dmask2 = rotate_mask(&dmask1);
        let dmask3 = rotate_mask(&dmask2);
        let dmask4 = rotate_mask(&dmask3);

        let count = fold_2d(input.as_slice(), &mask1)
            + fold_2d(input.as_slice(), &mask2)
            + fold_2d(input.as_slice(), &mask3)
            + fold_2d(input.as_slice(), &mask4)
            + fold_2d(input.as_slice(), &dmask1)
            + fold_2d(input.as_slice(), &dmask2)
            + fold_2d(input.as_slice(), &dmask3)
            + fold_2d(input.as_slice(), &dmask4);

        format!("{}", count)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mask1 = [
            [Some('M'), None, Some('S')],
            [None, Some('A'), None],
            [Some('M'), None, Some('S')],
        ];
        let mask2 = rotate_mask(&mask1);
        let mask3 = rotate_mask(&mask2);
        let mask4 = rotate_mask(&mask3);

        let count = fold_2d(input.as_slice(), &mask1)
            + fold_2d(input.as_slice(), &mask2)
            + fold_2d(input.as_slice(), &mask3)
            + fold_2d(input.as_slice(), &mask4);

        format!("{}", count)
    }
}

fn rotate_mask<const N: usize, const M: usize>(
    mask: &[[Option<char>; M]; N],
) -> [[Option<char>; N]; M] {
    std::array::from_fn(|i| std::array::from_fn(|j| mask[N - j - 1][i]))
}

fn fold_2d<const N: usize, const M: usize>(
    input: &[Vec<char>],
    mask: &[[Option<char>; M]; N],
) -> usize {
    if input.len() < mask.len() {
        return 0;
    }

    (0..=(input[0].len() - mask[0].len()))
        .filter(|&j| {
            input.iter().zip(mask.iter()).all(|(iline, &mline)| {
                iline.as_slice()[j..]
                    .iter()
                    .zip(mline.iter())
                    .all(|(&i, &m)| m.is_none_or(|m| i == m))
            })
        })
        .count()
        + fold_2d(&input[1..], mask)
}

gen_test!(
    a,
    Day04,
    r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    "18"
);

gen_test!(
    b,
    Day04,
    r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    "9"
);
