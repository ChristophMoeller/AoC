use super::*;
use iter_set::*;

// This solution makes some assumptions about the input.
// 1. The input has to be sorted by the card id.
// 2. There is no missing card id, i.e. the ids are 1,2,...,n.
// 3. The list of winning numbers and the list of numbers you have should not contain duplicates .
//    (Maybe the solution also works if a number appears twice in only one of the two lists.)

#[derive(Debug, Clone)]
pub struct Card {
    _id: u32, // It turns out, that the id is never really used
    winning: Vec<u32>,
    own: Vec<u32>,
}

impl Card {
    fn matches(mut self) -> usize {
        // Using the iter-set crate to determine the number of matches between the winning numbers and the numbers you have.
        // The iter-set crate works with sets that are represented as sortest iterators (without duplicates).
        self.winning.sort_unstable();
        self.own.sort_unstable();

        intersection(self.winning.iter(), self.own.iter()).count()
    }
}

pub struct Day04;
impl Solution for Day04 {
    type Input<'a> = Vec<usize>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        // Actually, the first and second part do not care about the card. The only relevant information is the number of matches.
        // Therefore, it is ok to discard any further information while parsing the input.
        parsing::parse(content)
            .unwrap()
            .1
            .into_iter()
            .map(|card| card.matches())
            .collect()
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let sum = input
            .iter()
            .map(|&matches| {
                if matches >= 1 {
                    2usize.pow(matches as u32 - 1)
                } else {
                    0
                }
            })
            .sum::<usize>();

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        // We store a list of tuples containing the number of matches a card has together with the amount of copies that we have.
        let mut cards = input
            .iter()
            .map(|&matches| (matches, 1usize))
            .collect::<Vec<_>>();

        for i in 0..cards.len() {
            let (matches, amount) = cards[i];
            for j in (i + 1)..=(i + matches) {
                cards[j].1 += amount;
            }
        }

        let sum = cards.iter().map(|(_, amount)| *amount).sum::<usize>();

        format!("{}", sum)
    }
}

mod parsing {
    use super::Card;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space0, space1, u32},
        multi::separated_list1,
        sequence::{delimited, separated_pair, tuple},
        *,
    };

    // "Card  x :"
    pub fn card_id(input: &str) -> IResult<&str, u32> {
        delimited(tuple((tag("Card"), space1)), u32, tuple((space0, tag(":"))))(input)
    }

    // "Card  x :        a b ... c  |   x y ... y"
    //  --------- ------ --------- ---  ---------
    //   card_id  space0  winning  sep     own
    pub fn card(input: &str) -> IResult<&str, Card> {
        separated_pair(
            card_id,
            space0,
            separated_pair(
                separated_list1(space1, u32),
                tuple((space0, tag("|"), space0)),
                separated_list1(space1, u32),
            ),
        )
        .map(|(id, (winning, own))| Card {
            _id: id,
            winning,
            own,
        })
        .parse(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Card>> {
        separated_list1(line_ending, card)(input)
    }
}

gen_test!(
    a,
    Day04,
    r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    "13"
);
gen_test!(
    b,
    Day04,
    r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    "30"
);
