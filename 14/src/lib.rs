use std::fs;
use std::collections::HashMap;

mod rules;

pub fn solve_pt1(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Excuse me there, but I can't help but notice that the filename you gave me is not a valid filename.");
    let mut lines = contents.lines();

    let mut polymer = lines.next().unwrap().to_string();
    lines.next();

    let mut rules = HashMap::new();
    for line in lines {
        rules::add_rule(&mut rules, line);
    }

    // println!("Initial polymer: {polymer}");
    for i in 1..=10 {
        polymer = rules::apply_rules(&polymer, &rules);
        // println!("After {i} iterations: {polymer}");
    }

    // Find the most common character and least common character.
    let mut counts = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Just need the counts of the most common and least common.
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn solve_pt2(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Excuse me there, but I can't help but notice that the filename you gave me is not a valid filename.");
    let mut lines = contents.lines();

    let mut polymer = lines.next().unwrap().to_string();
    lines.next();

    let mut rules = HashMap::new();
    for line in lines {
        rules::add_rule(&mut rules, line);
    }

    // println!("Initial polymer: {polymer}");
    for i in 1..=40 {
        polymer = rules::apply_rules(&polymer, &rules);
        // println!("After {i} iterations: {polymer}");
    }

    // Find the most common character and least common character.
    let mut counts = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Just need the counts of the most common and least common.
    counts.values().max().unwrap() - counts.values().min().unwrap()

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
        assert_eq!(solve_pt2("demo.txt"), 0);
    }
}