use std::fs;
use std::io::{self, BufRead};
use std::{thread, time};

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Create from a string in the form "x,y"
    fn new(s: &str) -> Point {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();

        Point { x, y }
    }
}

#[derive(Copy, Clone)]
struct Segment {
    // Not necessarily smaller than end.
    start: Point,
    end: Point,

    // For iteration
    current: Point,
    next: Point,
}

impl Segment {
    // Create from a string in the form "x1,y1 -> x2,y2"
    fn new(s: &str) -> Segment {
        let mut parts = s.split(" -> ");
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        Segment {
            start: Point::new(start),
            end: Point::new(end),
            current: Point { x: 0, y: 0 },
            next: Point::new(start),
        }
    }

    #[allow(dead_code)]
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    #[allow(dead_code)]
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl Iterator for Segment {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x == self.end.x && self.current.y == self.end.y {
            return None;
        }

        self.current = self.next;

        // Right now this will loop forever if the segment is not a perfect diagonal.
        self.next.x += (self.end.x - self.start.x).signum();
        self.next.y += (self.end.y - self.start.y).signum();

        Some(self.current)
    }
}

pub fn solve_pt1(filename: &str) -> i32 {
    let file = fs::File::open(filename).expect("Could not open file");

    let segments: Vec<Segment> = io::BufReader::new(file)
        .lines()
        .map(|line| Segment::new(&line.unwrap()))
        // // Filter out segments that are not horizontal or vertical
        // .filter(|s| s.is_horizontal() || s.is_vertical())
        .collect();

    let all_x = segments.iter().flat_map(|s| vec![s.start.x, s.end.x]);
    let all_y = segments.iter().flat_map(|s| vec![s.start.y, s.end.y]);
    let min = Point { x: all_x.clone().min().unwrap(), y: all_y.clone().min().unwrap() };
    let max = Point { x: all_x.max().unwrap(), y: all_y.max().unwrap() };

    let mut grid = vec![vec![0; (max.x - min.x + 1) as usize]; (max.y - min.y + 1) as usize];

    for segment in segments {
        for p in segment {
            let p = Point { x: p.x - min.x, y: p.y - min.y };
            grid[p.y as usize][p.x as usize] += 1;
        }

        // print_anim_frame(&grid);
    }

    // Count overlaps
    grid.iter().flat_map(|row| row.iter()).filter(|&&x| x > 1).count() as i32
}

#[allow(dead_code)]
fn print_anim_frame(grid: &Vec<Vec<i32>>) {
    // Clear the console
    print!("{}[2J", 27 as char);
    // Print the grid
    print_grid(grid);
    // Wait a bit
    thread::sleep(time::Duration::from_millis(500));
}

// A quick debug function to print the grid
fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}