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
        .map(|b| b.score())
        .sum()
}

pub fn solve_pt2(filename: &str) -> i32 {
    0
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
        assert_eq!(solve_pt2("demo.txt"), 0);
    }
}
