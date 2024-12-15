pub mod parser;
pub mod utils;

#[derive(Clone, Copy, Debug)]
enum Part {
    A,
    B,
}

impl Part {
    fn to_string(&self) -> &'static str {
        match *self {
            Part::A => "a",
            Part::B => "b",
        }
    }
}

pub trait Solution {
    type Input<'a>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a>;

    fn part_a<'a>(input: &Self::Input<'a>) -> String;
    fn part_b<'a>(input: &Self::Input<'a>) -> String;

    fn solve(content: &str, part_a: bool, part_b: bool) {
        let input = Self::parse(content);

        let solve_internal = |part: Part| {
            println!("");
            println!("Solving part {} ...", part.to_string());
            let timer = std::time::Instant::now();
            let solution = match part {
                Part::A => Self::part_a(&input),
                Part::B => Self::part_b(&input),
            };
            let duration = timer.elapsed();
            println!("... took {:?}", duration);
            println!("");
            println!("The Solution is:");
            println!("{solution}");
            println!("");
        };

        if part_a {
            solve_internal(Part::A)
        }
        if part_b {
            solve_internal(Part::B)
        }
    }
}

macro_rules! gen_solve {

    ( $(($x:expr, $m:ident, $d:ident)),* ) => {

        $(mod $m;)*

        pub fn solve(day: u32, part_a: bool, part_b: bool) {
            let content = std::fs::read_to_string(&format!("input/day{:0>2}.txt", day)).expect("unable to open input file");
            match day {
                $($x => {$m::$d::solve(&content, part_a, part_b);} )*
                _ => {unimplemented!();}
            }
        }
    };
}

macro_rules! gen_test {
    (a, $d:ident, $input:expr, $output:expr) => {
        #[cfg(test)]
        #[test]
        fn test_part_a() {
            let input = $d::parse($input);
            let output = $output;
            let result = $d::part_a(&input);
            assert_eq!(&result, output);
        }
    };
    (b, $d:ident, $input:expr, $output:expr) => {
        #[test]
        #[cfg(test)]
        fn test_part_b() {
            let input = $d::parse($input);
            let output = $output;
            let result = $d::part_b(&input);
            assert_eq!(&result, output);
        }
    };
}

gen_solve!(
    (01, day01, Day01),
    (02, day02, Day02),
    (03, day03, Day03),
    (04, day04, Day04),
    (05, day05, Day05),
    (09, day09, Day09),
    (10, day10, Day10),
    (11, day11, Day11),
    (12, day12, Day12),
    (13, day13, Day13),
    (14, day14, Day14),
    (15, day15, Day15)
);
