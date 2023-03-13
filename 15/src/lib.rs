use std::{fs, cmp::Reverse};
use priority_queue::PriorityQueue;

mod search;

use search::SearchState;

fn parse_grid(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("A message for you: Your file sucks.");

    contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn astar_search(grid: &Vec<Vec<i32>>) -> u64 {
    let mut to_visit = PriorityQueue::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let dest = (grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    // Initial state
    let state = SearchState::new(0, 0, 0);
    to_visit.push(state, Reverse(state.search_score(dest)));

    while let Some((state, Reverse(_))) = to_visit.pop() {
        if visited[state.y as usize][state.x as usize] {
            continue;
        }

        visited[state.y as usize][state.x as usize] = true;

        if state.x == dest.0 && state.y == dest.1 {
            return state.dist;
        }

        for neighbor in state.neighbors(grid) {
            to_visit.push(neighbor, Reverse(neighbor.search_score(dest)));
        }
    }

    0
}

pub fn solve_pt1(filename: &str) -> u64 {
    let grid = parse_grid(filename);
    astar_search(&grid)
}

pub fn solve_pt2(filename: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 40);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 0);
    }
}
