use advent2021_12::*;
use std::env;

fn main() {
    // Get the path to the file to read
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }
    let filename = &args[1];

    let result = solve_pt1(filename);
    println!("Part 1: {result}");

    let result = solve_pt2(filename);
    println!("Part 2: {result}");
}
