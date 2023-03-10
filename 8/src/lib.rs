#![allow(dead_code)]

mod signal;
mod decoding;
mod scenario;

use std::fs;
use crate::scenario::Scenario;

// Parse lines in the file like this:
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
fn parse(filename: &str) -> Vec<Scenario> {
    let contents = fs::read_to_string(filename).expect("Could not read file");

    contents
        .split('\n')
        .filter_map(Scenario::from_line)
        .collect()
}

pub fn solve_pt1(filename: &str) -> i32 {
    let scenarios = parse(filename);

    scenarios
        .iter()
        .flat_map(|s| &s.reading)
        .map(|r| r.possible_value_from_length().len())
        .filter(|&x| x == 1)
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_pt2(filename: &str) -> i32 {
    let scenarios = parse(filename);

    let mut sum = 0;

    for (i, scenario) in scenarios.iter().enumerate() {
        println!("Scenario {}", i);
        sum += scenario.reading_as_int();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 26);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 61229);
    }
}
