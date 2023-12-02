use super::*;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

pub struct Day02;
impl Solution for Day02 {
    type Input<'a> = Vec<Game>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let mut sum = 0;

        'outer: for game in input {
            for round in &game.rounds {
                if round.red > 12 || round.green > 13 || round.blue > 14 {
                    continue 'outer;
                }
            }
            sum += game.id;
        }

        format!("{}", sum)
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        let mut sum = 0u64;

        for game in input {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for round in &game.rounds {
                max_red = max_red.max(round.red);
                max_blue = max_blue.max(round.blue);
                max_green = max_green.max(round.green);
            }
            sum += (max_red * max_blue * max_green) as u64;
        }

        format!("{}", sum)
    }
}

mod parsing {
    use super::Color;
    use super::Game;
    use super::Round;
    use nom::character::complete::line_ending;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::tuple;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space1, u32},
        sequence::separated_pair,
        *,
    };

    fn color(input: &str) -> IResult<&str, (u32, Color)> {
        separated_pair(
            u32,
            space1,
            alt((
                tag("blue").map(|_| Color::Blue),
                tag("red").map(|_| Color::Red),
                tag("green").map(|_| Color::Green),
            )),
        )(input)
    }

    fn round(input: &str) -> IResult<&str, Round> {
        let (input, parts) = separated_list1(tag(", "), color)(input)?;
        let mut res = Round::default();

        for (amount, color) in parts {
            match color {
                Color::Red => {
                    res.red = amount;
                }
                Color::Blue => {
                    res.blue = amount;
                }
                Color::Green => {
                    res.green = amount;
                }
            }
        }

        Ok((input, res))
    }

    fn game_list(input: &str) -> IResult<&str, Vec<Round>> {
        separated_list1(tag("; "), round)(input)
    }

    fn game_id(input: &str) -> IResult<&str, u32> {
        delimited(tag("Game "), u32, tag(": "))(input)
    }

    fn game(input: &str) -> IResult<&str, Game> {
        tuple((game_id, game_list))
            .map(|(id, list)| Game { id, rounds: list })
            .parse(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(line_ending, game)(input)
    }
}

gen_test!(
    a,
    Day02,
    r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    "8"
);
gen_test!(
    b,
    Day02,
    r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    "2286"
);
