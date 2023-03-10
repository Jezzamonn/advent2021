
use once_cell::sync::Lazy;

// Which segments are on in the seven-segment display for each number, where segments are labeled a through g
const UNSCRAMBLED_SIGNALS_LETTERS: &str =
    "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";

static UNSCRAMBLED_SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    UNSCRAMBLED_SIGNALS_LETTERS
        .split(' ')
        .map(Signal::from_string)
        .collect()
});

#[derive(PartialEq)]
pub struct Signal(pub [bool; 7]);

impl Signal {
    pub fn empty() -> Self {
        Signal([false; 7])
    }

    pub fn from_string(s: &str) -> Self {
        // Treat each letter as a digit. E.g. a -> 0, b -> 1, etc.
        let mut signal = [false; 7];
        for c in s.chars() {
            signal[(c as i32 - 'a' as i32) as usize] = true;
        }
        Signal(signal)
    }

    pub fn to_ascii(&self) -> String {
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
    pub fn possible_value_from_length(&self) -> Vec<i32> {
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
    pub fn as_digit(&self) -> Option<i32> {
        UNSCRAMBLED_SIGNALS.iter().position(|s| s == self).map(|i| i as i32)
    }
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(i, _)| (i as i32 + 'a' as i32) as u8 as char)
                .collect::<String>()
        )
    }
}
