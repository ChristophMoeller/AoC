use clap::Parser;

/// Solutions to Advent of Code 2024
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// Print the solution to part a
    #[arg(short = 'a')]
    solve_a: bool,

    /// Print the solution to part b
    #[arg(short = 'b')]
    solve_b: bool,

    /// The day specifying the problem
    day: u32,
}

fn main() {
    let mut args = Args::parse();
    if !args.solve_a && !args.solve_b {
        args.solve_a = true;
        args.solve_b = true;
    }

    year2024::solve(args.day, args.solve_a, args.solve_b);
}
