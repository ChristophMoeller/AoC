use std::cmp::Ordering;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn index(&self) -> usize {
        match *self {
            Card::Number(d) => (d - 2) as usize,
            Card::Jack => 9 as usize,
            Card::Queen => 10 as usize,
            Card::King => 11 as usize,
            Card::Ace => 12 as usize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn type_of_hand(cards: &[Card; 5]) -> Type {
        let mut amount = [0; 13];

        for card in cards {
            amount[card.index()] += 1;
        }

        for n in amount {
            if n == 5 {
                return Type::FiveOfAKind;
            }
            if n == 4 {
                return Type::FourOfAKind;
            }
            if n == 3 {
                for m in amount {
                    if m == 2 {
                        return Type::FullHouse;
                    }
                }

                return Type::ThreeOfAKind;
            }
        }

        let mut pairs = 0;
        for n in amount {
            if n == 2 {
                pairs += 1;
            }
        }
        if pairs == 2 {
            return Type::TwoPair;
        }
        if pairs == 1 {
            return Type::OnePair;
        }

        Type::HighCard
    }

    fn type_of_hand_joker(cards: &[Card; 5]) -> Type {
        let pos = cards.iter().position(|x| *x == Card::Jack);

        let Some(pos) = pos else {
            return Card::type_of_hand(cards);
        };

        let cards_excepts_joker = [
            Card::Ace,
            Card::King,
            Card::Queen,
            Card::Number(10),
            Card::Number(9),
            Card::Number(8),
            Card::Number(7),
            Card::Number(6),
            Card::Number(5),
            Card::Number(4),
            Card::Number(3),
            Card::Number(2),
        ];

        let mut cards = cards.clone();
        let mut max_type = Type::HighCard;
        for card in cards_excepts_joker {
            cards[pos] = card;
            let t = Self::type_of_hand_joker(&cards);
            max_type = max_type.max(t);
        }

        max_type
    }

    fn compare_hands(a: &[Card; 5], b: &[Card; 5]) -> std::cmp::Ordering {
        Card::type_of_hand(a).cmp(&Card::type_of_hand(b)).then(
            a.iter()
                .zip(b.iter())
                .fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp(b))),
        )
    }
}

impl Card {
    fn cmp_joker(&self, other: &Card) -> Ordering {
        if *self == Card::Jack || *other == Card::Jack {
            if self == other {
                Ordering::Equal
            } else if *self == Card::Jack {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            self.cmp(other)
        }
    }

    fn compare_hands_joker(a: &[Card; 5], b: &[Card; 5]) -> std::cmp::Ordering {
        Card::type_of_hand_joker(a)
            .cmp(&Card::type_of_hand_joker(b))
            .then(
                a.iter()
                    .zip(b.iter())
                    .fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp_joker(b))),
            )
    }
}

pub struct Day07;
impl Solution for Day07 {
    type Input<'a> = Vec<([Card; 5], u32)>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut hand = input.clone();

        hand.sort_by(|(x, _), (y, _)| Card::compare_hands(x, y));

        let sum: u64 = hand
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u64 + 1) * (*bid as u64))
            .sum();

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut hand = input.clone();

        hand.sort_by(|(x, _), (y, _)| Card::compare_hands_joker(x, y));

        let sum: u64 = hand
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u64 + 1) * (*bid as u64))
            .sum();

        format!("{}", sum)
    }
}

mod parsing {
    use super::Card;

    use nom::{
        bytes::complete::take_while1,
        character::complete::{anychar, line_ending, space1, u32},
        multi::{many1, separated_list1},
        sequence::separated_pair,
        *,
    };

    fn card(input: &str) -> IResult<&str, Card> {
        anychar
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Number(10),
                '9' => Card::Number(9),
                '8' => Card::Number(8),
                '7' => Card::Number(7),
                '6' => Card::Number(6),
                '5' => Card::Number(5),
                '4' => Card::Number(4),
                '3' => Card::Number(3),
                '2' => Card::Number(2),
                _ => unimplemented!(),
            })
            .parse(input)
    }

    fn hand(input: &str) -> IResult<&str, [Card; 5]> {
        take_while1(|c: char| c.is_alphanumeric())
            .and_then(many1(card))
            .map(|x| {
                assert!(x.len() >= 5);
                [x[0], x[1], x[2], x[3], x[4]]
            })
            .parse(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<([Card; 5], u32)>> {
        separated_list1(line_ending, separated_pair(hand, space1, u32))(input)
    }
}

gen_test!(
    a,
    Day07,
    r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    "6440"
);
gen_test!(
    b,
    Day07,
    r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    "5905"
);
