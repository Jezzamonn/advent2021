use std::cmp::min;

use regex::Regex;

use crate::point::Point;

enum Axis {
    X,
    Y,
}

pub struct Fold {
    axis: Axis,
    position: i32,
}

impl Fold {
    // Parse a string like "fold along x=655", using regex.
    pub fn from_string(s: &str) -> Fold {
        let re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let axis = match &caps[1] {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("Invalid axis"),
        };
        let position = caps[2].parse::<i32>().unwrap();
        Fold { axis, position }
    }

    pub fn fold_point(&self, p: &Point) -> Point {
        match self.axis {
            Axis::X => Point {
                x: fold_value(p.x, self.position),
                y: p.y,
            },
            Axis::Y => Point {
                x: p.x,
                y: fold_value(p.y, self.position),
            },
        }
    }
}

/// Fold a value along a fold line.
///
/// Values < fold are unchanged.
/// Values > fold are folded to the other side of the fold line.
/// Undefined behavior if value == fold.
fn fold_value(value: i32, fold: i32) -> i32 {
    min(value, 2 * fold - value)
}

// Ascii diagram for me to figure things out:

// 0 1 2 3 4 5 6 7 8
// . . . . | . . . .
// 0 1 2 3 x 3 2 1 0

// 5 -> 3 = 4 - (5 - 4) = 2 * fold - value
// 6 -> 2 = 4 - (6 - 4)
// 7 -> 1 = 4 - (7 - 4)
// 8 -> 0 = 4 - (8 - 4)
