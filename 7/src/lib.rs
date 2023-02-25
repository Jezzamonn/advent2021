use std::fs;

fn parse_input(filename: &str) -> Vec<i32> {
    // Read the file to a string
    let contents = fs::read_to_string(filename).expect("Could not read file");

    contents
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn solve(filename: &str) -> i32 {
    // The position of each unit.
    let mut positions = parse_input(filename);
    positions.sort();

    // Start with the cost for moving to position 0.
    let mut cur_cost: i32 = positions.iter().sum();

    let mut min_cost = cur_cost;
    let mut units_to_move_forward = 0;

    let max_position = positions[positions.len() - 1];

    // Digrams to help me figure this out:

    // Positions: 3, 4, 5
    // (i = 0)
    // Cost: 3 + 4 + 5 = 12
    //
    // i = 1
    // Positions: 3, 4, 5
    // Index: 0
    // Cost: 2 + 3 + 4 = 9
    // Cost: Last cost - 3 + 0
    //
    // i = 2
    // Positions: 3, 4, 5
    // Index: 0
    // Cost: 1 + 2 + 3 = 6
    // Cost: Last cost - 3 + 0
    //
    // i = 3
    // Positions: 3, 4, 5
    // Index: 0
    // Cost: 0 + 1 + 2 = 3
    // Cost: Last cost - 3 + 0
    //
    // i = 4
    // Positions: 3, 4, 5
    // Index: 1
    // Cost: 1 + 0 + 1 = 2
    // Cost: Last cost - 2 + 1
    //
    // i = 5
    // Positions: 3, 4, 5
    // Index: 2
    // Cost: 2 + 1 + 0 = 3
    // Cost: Last cost - 1 + 2
    //
    // End loop.



    // Consider moving all units to all other positions.
    for i in 1..=max_position {
        // Update the index to point to the first unit that needs to move forward to get to position i
        // Doesn't include units that are already *at* position i.
        while positions[units_to_move_forward] < i {
            units_to_move_forward += 1;
        }
        // The cost increases for all the pieces that need to move forward (index)
        let units_to_move_backward = positions.len() - units_to_move_forward;
        cur_cost += units_to_move_forward as i32 - units_to_move_backward as i32;
        min_cost = min_cost.min(cur_cost);
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve("demo.txt"), 37);
    }
}