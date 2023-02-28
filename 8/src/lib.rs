use std::fs;
use once_cell::sync::Lazy;

// Which segments are on in the seven-segment display for each number, where segments are labeled a through g
const UNSCRAMBLED_SIGNALS_LETTERS: &str = "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";

static UNSCRAMBLED_SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    UNSCRAMBLED_SIGNALS_LETTERS
    .split(" ")
    .map(|s| Signal::from_string(s))
    .collect()
});

struct Signal ([bool; 7]);

impl Signal {
    fn from_string(s: &str) -> Self {
        // Treat each letter as a digit. E.g. a -> 0, b -> 1, etc.
        let mut signal = [false; 7];
        for c in s.chars() {
            signal[(c as i32 - 'a' as i32) as usize] = true;
        }
        Signal(signal)
    }

    fn to_string(&self) -> String {
        self.0.iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| (i as i32 + 'a' as i32) as u8 as char)
            .collect()
    }

    fn num_digits(&self) -> i32 {
        self.0.iter().filter(|&&b| b).count().try_into().unwrap()
    }

    /// Uses the number of on-segments to guess the value of the signal.
    fn possible_value_from_length(&self) -> Vec<i32> {
        let mut possible_values = Vec::new();

        for (i, signal) in UNSCRAMBLED_SIGNALS.iter().enumerate() {
            if signal.num_digits() == self.num_digits() {
                possible_values.push(i as i32);
            }
        }

        possible_values
    }
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
struct Scenario {
    sample_signals: Vec<Signal>,
    reading: Vec<Signal>,
}

impl Scenario {
    fn from_line(line: &str) -> Option<Self> {
        if line.len() == 0 {
            return Option::None;
        }

        let parts: Vec<&str> = line.split(" | ").collect();

        if parts.len() != 2 {
            panic!("Invalid line: {}", line);
        }

        Option::Some(Self {
            sample_signals: parts[0].split(" ").map(|s| Signal::from_string(s)).collect(),
            reading: parts[1].split(" ").map(|s| Signal::from_string(s)).collect(),
        })
    }
}

// Parse lines in the file like this:
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
fn parse(filename: &str) -> Vec<Scenario> {
    let contents = fs::read_to_string(filename).expect("Could not read file");

    contents
        .split('\n')
        .map(|line| Scenario::from_line(line))
        .filter_map(|s| s)
        .collect()
}

pub fn solve_pt1(filename: &str) -> i32 {
    let scenarios = parse(filename);

    scenarios.iter()
        .flat_map(|s| &s.reading)
        .map(|r| r.possible_value_from_length().len())
        .filter(|&x| x == 1)
        .count().try_into().unwrap()
}