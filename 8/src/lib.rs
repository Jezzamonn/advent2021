#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::fs;

// Which segments are on in the seven-segment display for each number, where segments are labeled a through g
const UNSCRAMBLED_SIGNALS_LETTERS: &str =
    "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";

static UNSCRAMBLED_SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    UNSCRAMBLED_SIGNALS_LETTERS
        .split(" ")
        .map(|s| Signal::from_string(s))
        .collect()
});

#[derive(PartialEq)]
pub struct Signal([bool; 7]);

impl Signal {
    fn empty() -> Self {
        Signal([false; 7])
    }

    fn from_string(s: &str) -> Self {
        // Treat each letter as a digit. E.g. a -> 0, b -> 1, etc.
        let mut signal = [false; 7];
        for c in s.chars() {
            signal[(c as i32 - 'a' as i32) as usize] = true;
        }
        Signal(signal)
    }

    fn to_string(&self) -> String {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| (i as i32 + 'a' as i32) as u8 as char)
            .collect()
    }

    fn to_ascii(&self) -> String {
        format!(
            "  {}  \n {}{}{} \n {}{}{} ",
            if self.0[0] { '_' } else { ' ' },
            if self.0[1] { '|' } else { ' ' },
            if self.0[3] { '_' } else { ' ' },
            if self.0[2] { '|' } else { ' ' },
            if self.0[4] { '|' } else { ' ' },
            if self.0[6] { '_' } else { ' ' },
            if self.0[5] { '|' } else { ' ' })
    }

    fn on_segments(&self) -> Vec<i32> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i as i32)
            .collect() // TODO: Can I remove this collect?
    }

    fn num_segments(&self) -> i32 {
        self.0.iter().filter(|&&b| b).count().try_into().unwrap()
    }

    /// Uses the number of on-segments to guess the value of the signal.
    fn possible_value_from_length(&self) -> Vec<i32> {
        let mut possible_values = Vec::new();

        for (i, signal) in UNSCRAMBLED_SIGNALS.iter().enumerate() {
            if signal.num_segments() == self.num_segments() {
                possible_values.push(i as i32);
            }
        }

        possible_values
    }

    /// If the segments of this signal match a known number, return that number. Otherwise return None.
    /// This is only really meaningful for unscrambled signals.
    fn as_digit(&self) -> Option<i32> {
        UNSCRAMBLED_SIGNALS.iter().position(|s| s == self).map(|i| i as i32)
    }
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


/// Once we've figured out the decoding, this will be a matrix where the
/// decoding[input][output] is true if the input segment decodes to output.
/// Once we've figured out the decoding, all the values will be Some, and
/// only one per row will be true.
struct Decoding ([[Option<bool>; 7]; 7]);

impl Decoding {
    fn new() -> Self {
        Decoding([[Option::None; 7]; 7])
    }

    fn from_sample_signals(sample_signals: &[Signal]) -> Self {
        // Logic: Sum of how frequently each segment appears. That lets us deduce most of the signals.
        // For the two ambiguous pairs, we can look at the 1 signal and the 4 signal to disambiguate.
        let appearances = sample_signals
            .iter()
            .fold([0; 7], |mut appearances, signal| {
                for (i, &b) in signal.0.iter().enumerate() {
                    if b {
                        appearances[i] += 1;
                    }
                }
                appearances
            });

        let mut decoding = [[Some(false); 7]; 7];

        let one_signal = sample_signals.iter().find(|s| s.possible_value_from_length()[0] == 1).unwrap();
        let four_signal = sample_signals.iter().find(|s| s.possible_value_from_length()[0] == 4).unwrap();

        for (i, &c) in appearances.iter().enumerate() {
            match c {
                4 => { // Corresponds to 'e'
                    decoding[i][4] = Some(true);
                }
                6 => { // Corresponds to 'b'
                    decoding[i][1] = Some(true);
                }
                7 => { // Corresponds to 'd' or 'g', use presence in four_signal to disambiguate
                    if four_signal.0[i] {
                        decoding[i][3] = Some(true);
                    } else {
                        decoding[i][6] = Some(true);
                    }
                }
                8 => { // Corresponds to 'a' or 'c', use presence in one_signal to disambiguate
                    if one_signal.0[i] {
                        decoding[i][2] = Some(true);
                    } else {
                        decoding[i][0] = Some(true);
                    }
                }
                9 => { // Corresponds to 'f'
                    decoding[i][5] = Some(true);
                }
                _ => { panic!("Unexpected number of appearances: {}", c); }
            }
        }

        Self(decoding)
    }

    /// Decodes a digit from a scrambled signal. The decoding matrix must be filled in first.
    fn unscramble_segment(&self, scrambled_segment: usize) -> usize {
        self.0[scrambled_segment]
            .iter()
            .enumerate()
            .filter(|(_, &b)| b.is_some() && b.unwrap())
            .map(|(i, _)| i)
            .next()
            .unwrap()
    }

    fn unscramble_signal(&self, scrambled_signal: &Signal) -> Signal {
        let mut decoded = Signal::empty();
        for scrambled_segment in 0..7 {
            let decoded_segment = self.unscramble_segment(scrambled_segment);
            decoded.0[decoded_segment] = scrambled_signal.0[scrambled_segment];
        }
        decoded
    }

    // /// The bulk of the logic for this section.
    // /// Looking at a scrambled digit, we can use it to rule out some possibilities for the decoding.
    // /// For example, only having two segments lit means it's a 1, and it means that those two segments decode to one of two possibilities,
    // /// so we can turn part of the mapping there into None, though we have to leave the rest as Some.
    // /// Based on what we currently know we should be able to fill out more and more of the decoding table until it's all full.
    // fn update_decoding_for_signal(&mut self, sample_signal: &Signal) {
    //     // TODO: Filter out possible values that are impossible based on what we know of the decoding.
    //     let possible_values = sample_signal.possible_value_from_length();
    //     if possible_values.len() == 1 {
    //         let value = possible_values[0];
    //         let unscrambled_signal = &UNSCRAMBLED_SIGNALS[value as usize];
    //         // This tells us information about which segments are on (but we don't know which of the on segments are which),
    //         // and which segments are off (but we don't know which of the off segments are which).
    //         // We can use this to fill in the decoding table.

    //         // TODO: Handle when there's multiple possible values, and extract the known on and off segments.
    //         let on_segments_in_input = sample_signal.on_segments();
    //         for i in on_segments_in_input {
    //             for o in 0..=7 {
    //                 if unscrambled_signal.0[o] {
    //                     continue;
    //                 }
    //                 self.0[i as usize][o] = Option::Some(false);
    //             }
    //         }
    //     }
    // }

    fn is_complete(&self) -> bool {
        self.0.iter().flat_map(|x| x).all(|x| x.is_some())
    }

    // Create an ascii table that looks something like this::
    //   0 1 2 3 4 5 6
    // 0 x . . . . ? ?
    // 1 . x . . . . .
    // 2 . . x . . . .
    // 3 . . . x . . .
    // 4 . . . . x . .
    // 5 . . . . . x .
    // 6 . . . . . . x
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("  0 1 2 3 4 5 6\n");
        for (i, row) in self.0.iter().enumerate() {
            s.push_str(&format!("{} ", i));
            for b in row {
                s.push_str(&format!("{} ", match b {
                    Some(true) => "x",
                    Some(false) => ".",
                    None => "?",
                }));
            }
            s.push_str(&format!("\n"));
        }
        s
    }
}

impl std::fmt::Debug for Decoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
struct Scenario {
    reading: Vec<Signal>,
    decoding: Decoding,
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

        let sample_signals: Vec<Signal> = parts[0]
            .split(" ")
            .map(|s| Signal::from_string(s))
            .collect();

        let decoding = Decoding::from_sample_signals(&sample_signals);

        Option::Some(Self {
            reading: parts[1]
                .split(" ")
                .map(|s| Signal::from_string(s))
                .collect(),
            decoding: decoding,
        })
    }

    fn decoded_reading(&self) -> Vec<Signal> {
        self.reading
            .iter()
            .map(|s| self.decoding.unscramble_signal(s))
            .collect()
    }

    fn reading_as_int(&self) -> i32 {
        self.decoded_reading()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, s)| s.as_digit().unwrap() * (10 as i32).pow(i as u32))
            .sum()
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

    scenarios
        .iter()
        .flat_map(|s| &s.reading)
        .map(|r| r.possible_value_from_length().len())
        .filter(|&x| x == 1)
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_pt2(filename: &str) -> i32 {
    let scenarios = parse(filename);

    let mut sum = 0;

    for (i, scenario) in scenarios.iter().enumerate() {
        println!("Scenario {}", i);
        println!("Decoding:\n{:?}", scenario.decoding);
        println!("Reading:\n");
        for r in &scenario.reading {
            println!("{}", r.to_ascii());
        }
        println!("Decoded reading:\n");
        for r in scenario.decoded_reading() {
            println!("{}", r.to_ascii());
        }
        sum += scenario.reading_as_int();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1("demo.txt"), 26);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2("demo.txt"), 61229);
    }
}
