use std::fs;

const SPAWN_TIME: i32 = 8;
const RESET_TIME: i32 = 6;
const MAX_INTERNAL_TIMER: i32 = SPAWN_TIME;

fn parse_input(filename: &str) -> [i32; MAX_INTERNAL_TIMER as usize + 1] {
    // Read the file to a string
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let internal_timers = contents
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap());

    to_fish_per_internal_timer(internal_timers)
}

fn to_fish_per_internal_timer(
    internal_timers: impl Iterator<Item = i32>,
) -> [i32; MAX_INTERNAL_TIMER as usize + 1] {
    let mut num_fish_per_internal_timer = [0; MAX_INTERNAL_TIMER as usize + 1];

    for internal_timer in internal_timers {
        num_fish_per_internal_timer[internal_timer as usize] += 1;
    }

    num_fish_per_internal_timer
}

fn simulate_generation(num_fish_per_internal_timer: &mut [i32]) {
    let reseting_fish = num_fish_per_internal_timer[0];
    let new_fish = num_fish_per_internal_timer[0];

    for i in 1..=MAX_INTERNAL_TIMER {
        num_fish_per_internal_timer[(i - 1) as usize] = num_fish_per_internal_timer[i as usize];
    }
    num_fish_per_internal_timer[MAX_INTERNAL_TIMER as usize] = 0;

    num_fish_per_internal_timer[RESET_TIME as usize] += reseting_fish;
    num_fish_per_internal_timer[SPAWN_TIME as usize] += new_fish;
}

/// Prints an ascii table of the fish population
#[allow(dead_code)]
fn print_generations(generation_number: i32, num_fish_per_internal_timer: &[i32]) {
    print!(
        "{}",
        format_generations(generation_number, num_fish_per_internal_timer).as_str()
    );

    // Print a separating line
    print!("        ");
    for _ in 0..=MAX_INTERNAL_TIMER {
        print!("---");
    }
    println!();
}

fn format_generations(generation_number: i32, num_fish_per_internal_timer: &[i32]) -> String {
    let mut result = String::new();

    result.push_str(&format!("Gen {:2}: ", generation_number));
    for i in (0..=MAX_INTERNAL_TIMER).rev() {
        result.push_str(&format!("{:2} ", i));
    }
    result.push_str("\n");

    result.push_str("        ");
    for i in (0..=MAX_INTERNAL_TIMER).rev() {
        result.push_str(&format!("{:2} ", num_fish_per_internal_timer[i as usize]));
    }
    result.push_str("\n");

    result
}

pub fn solve_pt1(filename: &str, num_generations: i32) -> i32 {
    let mut num_fish_per_internal_timer = parse_input(filename);

    // print_generations(0, &num_fish_per_internal_timer);

    #[allow(unused_variables)]
    for i in 0..num_generations {
        simulate_generation(&mut num_fish_per_internal_timer);
        // print_generations(i + 1, &num_fish_per_internal_timer);
    }

    return num_fish_per_internal_timer.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        // List of fishes and their internal timers
        let demo_results = vec![
            vec![3, 4, 3, 1, 2],
            vec![2, 3, 2, 0, 1],
            vec![1, 2, 1, 6, 0, 8],
            vec![0, 1, 0, 5, 6, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 7, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 7],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5],
            vec![1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8],
            vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 0, 4, 5, 6, 6, 6, 7, 7, 8, 8],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6, 3, 4, 5, 5, 5, 6, 6, 7, 7, 8],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7],
            vec![
                1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
            ],
            vec![
                0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
            ],
            vec![
                6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
            ],
        ];

        for (i, expected) in demo_results.iter().enumerate() {
            let mut num_fish_per_internal_timer = parse_input("demo.txt");
            for _ in 0..i {
                simulate_generation(&mut num_fish_per_internal_timer);
            }
            let expected = to_fish_per_internal_timer(expected.iter().copied());
            assert_eq!(
                num_fish_per_internal_timer,
                expected,
                "Mismatch:\nleft:\n{}\nright:\n{}",
                format_generations(i as i32, &num_fish_per_internal_timer),
                format_generations(i as i32, &expected)
            );
        }
    }
}
