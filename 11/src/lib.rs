use std::fs;

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const FLASHED_MASK: u8 = 1 << 7;

fn parse(filename: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|x| x as u8))
                .collect()
        })
        .collect()
}

fn increase_energy_levels(grid: &mut [Vec<u8>]) {
    grid.iter_mut()
        .for_each(|r| r.iter_mut().for_each(|x| *x += 1));
}

fn flash(grid: &mut [Vec<u8>]) -> u64 {
    let mut num_flashed = 0;
    let mut to_flash: Vec<(usize, usize)> = Vec::new();

    to_flash.extend(grid.iter().enumerate().flat_map(|(y, r)| {
        r.iter()
            .enumerate()
            .filter(|(_, &o)| o >= 10)
            .map(move |(x, _)| (x, y))
    }));

    while let Some((x, y)) = to_flash.pop() {
        // Check if already flashed.
        if grid[y][x] & FLASHED_MASK > 0 {
            continue;
        }

        // Flash!
        num_flashed += 1;

        // Increase neighbours
        for (dx, dy) in DELTAS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            grid[ny][nx] += 1;
            if grid[ny][nx] >= 10 {
                // Add to the to_flash list. The check for if it's already flashed will be done when it's popped from the list.
                to_flash.push((nx, ny));
            }
        }

        // Mark as flashed
        grid[y][x] |= 1 << 7;
    }

    num_flashed
}

fn reset_flashed(grid: &mut [Vec<u8>]) {
    grid.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|x| {
            if *x > 10 {
                *x = 0
            }
        })
    });
}

fn simulate_step(grid: &mut [Vec<u8>]) -> u64 {
    increase_energy_levels(grid);

    let num_flashed = flash(grid);

    reset_flashed(grid);

    num_flashed
}

pub fn solve_pt1(filename: &str) -> u64 {
    let mut grid = parse(filename);

    (0..100).map(|_| simulate_step(&mut grid)).sum()
}

pub fn solve_pt2(filename: &str) -> u64 {
    let mut grid = parse(filename);
    let size = (grid.len() * grid[0].len()) as u64;

    (1..).map(|i| (i, simulate_step(&mut grid))).find(|(_, x)| *x == size).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 1656);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 195);
    }
}
