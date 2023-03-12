use std::fs;

mod rules;
mod pairs;

use rules::Rules;
use pairs::Pairs;

fn solve(filename: &str, num_steps: i32) -> u64 {
    let contents = fs::read_to_string(filename).expect("Excuse me there, but I can't help but notice that the filename you gave me is not a valid filename.");
    let mut lines = contents.lines();

    let mut pairs = lines.next().map(Pairs::from_string).unwrap();
    lines.next();

    let rules = Rules::from_lines(&mut lines);

    for _ in 1..=num_steps {
        pairs = rules.apply_rules_to_pairs(pairs);
    }

    let counts = pairs.count_chars();

    // Just need the counts of the most common and least common.
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn solve_pt1(filename: &str) -> u64 {
    solve(filename, 10)
}

pub fn solve_pt2(filename: &str) -> u64 {
    solve(filename, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 1588);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 2188189693529);
    }
}