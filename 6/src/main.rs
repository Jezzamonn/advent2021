use advent2021_6::solve_pt1;
use std::env;

fn main() {
    // Get the path to the file to read
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <filename> <num_generations>", args[0]);
    }
    let filename = &args[1];
    let num_generations = args[2].parse::<i32>().unwrap();

    let result = solve_pt1(filename, num_generations);
    println!("Part 1: {result}");
}
