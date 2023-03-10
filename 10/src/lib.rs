use std::fs;

mod bracket;

use bracket::Bracket;

fn first_invalid_bracket(line: &str) -> Option<Bracket> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if let Some(b) = Bracket::from_opening_char(c) {
            stack.push(b);
        }
        else if let Some(b) = Bracket::from_closing_char(c) {
            if let Some(top) = stack.pop() {
                if top != b {
                    return Some(b);
                }
            }
            else {
                return Some(b);
            }
        }
    }

    None
}

pub fn solve_pt1(filename: &str) -> i32 {
    // Read file into a string
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
        .lines()
        .filter_map(first_invalid_bracket)
        .map(|b| b.score_if_invalid())
        .sum()
}

/// Gets the brackets that don't have a closing bracket.
///
/// If an invalid closing bracket is found, the line is invalid so we return None.
fn get_unmatched_brackets(line: &str) -> Option<Vec<Bracket>> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if let Some(b) = Bracket::from_opening_char(c) {
            stack.push(b);
        }
        else if let Some(b) = Bracket::from_closing_char(c) {
            if let Some(top) = stack.pop() {
                if top != b {
                    return None;
                }
            }
            else {
                return None;
            }
        }
    }

    Some(stack)
}

pub fn solve_pt2(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut scores: Vec<u64> = contents
        .lines()
        .filter_map(get_unmatched_brackets)
        .map(|brackets| brackets.iter().rev().fold(0, |acc, b| 5 * acc + b.score_if_unmatched() as u64))
        .collect();

    scores.sort();
    // Problem guarantees there will be an odd number of scores.
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 26397);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 288957);
    }
}
