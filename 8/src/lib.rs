use std::fs;
use once_cell::sync::Lazy;

// Which segments are on in the seven-segment display for each number, where segments are labeled a through g
const UNSCRAMBLED_SIGNALS_LETTERS: &str = "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";

static UNSCRAMBLED_SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    UNSCRAMBLED_SIGNALS_LETTERS
    .split(" ")
    .map(|s| Signal::fromString(s))
    .collect()
});

struct Signal ([bool; 7]);

impl Signal {
    fn fromString(s: &str) -> Self {
        // Treat each letter as a digit. E.g. a -> 0, b -> 1, etc.
        let mut signal = [false; 7];
        for c in s.chars() {
            signal[(c as i32 - 'a' as i32) as usize] = true;
        }
        Signal(signal)
    }

    fn toString(&self) -> String {
        self.0.iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| (i as i32 + 'a' as i32) as u8 as char)
            .collect()
    }

    /// Uses the number of on-segments to guess the value of the signal.
    fn possible_value_from_length(&self) -> Vec<i32> {
        let mut possible_values = Vec::new();

        for (i, signal) in UNSCRAMBLED_SIGNALS.iter().enumerate() {
            if signal.0.len() == self.0.len() {
                possible_values.push(i as i32);
            }
        }

        possible_values
    }
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.toString())
    }
}

#[derive(Debug)]
struct Scenario {
    sample_signals: Vec<Signal>,
    reading: Vec<Signal>,
}

impl Scenario {
    fn fromLine(line: &str) -> Option<Self> {
        if line.len() == 0 {
            return Option::None;
        }

        let parts: Vec<&str> = line.split(" | ").collect();

        if parts.len() != 2 {
            panic!("Invalid line: {}", line);
        }

        Option::Some(Self {
            sample_signals: parts[0].split(" ").map(|s| Signal::fromString(s)).collect(),
            reading: parts[1].split(" ").map(|s| Signal::fromString(s)).collect(),
        })
    }
}

// Parse lines in the file like this:
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
fn parse(filename: &str) -> Vec<Scenario> {
    let contents = fs::read_to_string(filename).expect("Could not read file");

    contents
        .split('\n')
        .map(|line| Scenario::fromLine(line))
        .filter_map(|s| s)
        .collect()
}

pub fn solve_pt1(filename: &str) -> i32 {
    let scenarios = parse(filename);

    for scenario in scenarios {
        println!("Scenario: {:?}", scenario);
    }

    0
}