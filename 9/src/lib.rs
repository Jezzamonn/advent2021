mod grid;

use grid::Grid;
use std::{thread, time::Duration};

const DELTAS: [(i32, i32); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

fn parse_input(filename: &str) -> Grid {
    println!("{:?}", std::env::current_dir().unwrap());
    let contents = std::fs::read_to_string(filename).unwrap();
    Grid::from_string(&contents)
}

pub fn solve_pt1(filename: &str) -> u32 {
    let grid = parse_input(filename);

    // println!("{:?}", grid);

    let mut risk = 0;

    for x in -1..=(grid.w as i32) {
        for y in -1..=(grid.h as i32) {
            let z = grid.get(x, y);
            let is_lowest = DELTAS.iter()
                .map(|(dy, dx)| grid.get(x + dx, y + dy) > z).all(|b| b);
            if is_lowest {
                risk += z + 1
            }
        }
    }

    risk
}

fn find_basin_size(grid: &Grid, visited: &mut [Vec<bool>], x: i32, y: i32) -> i32 {
    let mut to_visit = vec![(x, y)];

    let mut basin_size = 0;

    while let Some((x, y)) = to_visit.pop() {
        if !grid.in_bounds(x, y) {
            continue;
        }

        if visited[y as usize][x as usize] {
            continue;
        }

        // Get the elevation of this cell
        let z = grid.get(x, y);

        // Too high to be part of the basin
        if z >= 9 {
            continue;
        }

        // Mark this cell as visited
        visited[y as usize][x as usize] = true;
        basin_size += 1;

        // Get the neighbors of this cell. Just add them all for now, we'll do the filtering when considering points.
        let neighbors = DELTAS.iter()
            .map(|(dy, dx)| (x + dy, y + dx));

        to_visit.extend(neighbors);

        // Print the frame
        // print_frame(grid, visited, &to_visit);
    }

    basin_size
}

/// Prints one frame of an animation to the terminal.
#[allow(unused)]
fn print_frame(grid: &Grid, visited: &[Vec<bool>], to_visit: &[(i32, i32)]) {
    // Clear the screen
    print!("\x1b[2J");

    // Print the grid
    print_grid(grid, visited, to_visit);

    // Sleep for a bit
    thread::sleep(Duration::from_millis(150));
}

/// Prints the grid, using terminal colors to represent whether a cell is visited, or in the to_visit list.
fn print_grid(grid: &Grid, visited: &[Vec<bool>], to_visit: &[(i32, i32)]) {
    for y in 0..=(grid.h as i32 - 1) {
        for x in 0..=(grid.w as i32 - 1) {
            let z = grid.get(x, y);
            let is_visited = visited[y as usize][x as usize];
            let is_to_visit = to_visit.iter().any(|(tx, ty)| tx == &x && ty == &y);

            if is_to_visit {
                // Print in yellow
                print!("\x1b[1;33m{}\x1b[0m", z);
            } else if is_visited {
                // Print in green
                print!("\x1b[1;32m{}\x1b[0m", z);
            } else {
                print!("{}", z);
            }
        }
        println!();
    }
}

pub fn solve_pt2(filename: &str) -> i32 {
    let grid = parse_input(filename);

    let mut basin_sizes: Vec<i32> = vec![];
    let mut visited = vec![vec![false; grid.w]; grid.h];

    for x in 0..=(grid.w as i32 - 1) {
        for y in 0..=(grid.h as i32 - 1) {
            // Do a search to find the basin size
            let basin_size = find_basin_size(&grid, &mut visited, x, y);
            if basin_size > 0 {
                basin_sizes.push(basin_size);
            }
        }
    }

    // println!("{:?}", basin_sizes);

    basin_sizes.sort();

    // Get the 3 largest basin sizes
    let largest_basin_sizes = basin_sizes.iter().rev().take(3);
    largest_basin_sizes.product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 15);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 1134);
    }

}
