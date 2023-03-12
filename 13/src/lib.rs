use std::{fs, collections::HashSet};

mod point;
mod fold;

use point::Point;
use fold::Fold;

pub fn solve_pt1(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Eyyy, something's wrong with ya input file ya dummy!");

    let mut lines = contents.lines();
    let dots: HashSet<_> = lines.by_ref().take_while(|l| !l.is_empty()).map(Point::from_string).collect();

    let first_fold = lines.next().map(Fold::from_string).unwrap();
    let dots = dots.iter().map(|p| first_fold.fold_point(p)).collect::<HashSet<_>>();

    dots.len() as u64
}

/// Returns an ascii diagram of all the dots in the hashset.
fn dots_to_string(dots: &HashSet<Point>) -> String {
    let max_x = dots.iter().map(|p| p.x).max().unwrap();
    let max_y = dots.iter().map(|p| p.y).max().unwrap();

    let mut result = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&Point { x, y }) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

pub fn solve_pt2(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Eyyy, something's wrong with ya input file ya dummy!");

    let mut lines = contents.lines();
    let mut dots: HashSet<_> = lines.by_ref().take_while(|l| !l.is_empty()).map(Point::from_string).collect();

    for fold in lines.map(Fold::from_string) {
        dots = dots.iter().map(|p| fold.fold_point(p)).collect::<HashSet<_>>();
    }

    dots_to_string(&dots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 17);
    }
}