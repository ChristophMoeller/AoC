use clap::Parser;

/// Solutions to Advent of Code 2023
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

    year2023::solve(args.day, args.solve_a, args.solve_b);

    //     let mut game = year2023::fun::GameOfLife::new(
    //         r"..x...
    // ...x..
    // .xxx..
    // ......
    // ......
    // ......",
    //     );

    //     println!("{game}");
    //     for _ in 0..100 {
    //         game.step();
    //         println!("{game}");
    //         std::thread::sleep(std::time::Duration::from_millis(100));
    //     }
}
