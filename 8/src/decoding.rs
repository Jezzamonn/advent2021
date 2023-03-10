use crate::signal::Signal;

/// Once we've figured out the decoding, this will be a matrix where the
/// decoding[input][output] is true if the input segment decodes to output.
/// Once we've figured out the decoding, all the values will be Some, and
/// only one per row will be true.
pub struct Decoding ([[Option<bool>; 7]; 7]);

impl Decoding {
    fn new() -> Self {
        Decoding([[Option::None; 7]; 7])
    }

    pub fn from_sample_signals(sample_signals: &[Signal]) -> Self {
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

    pub fn unscramble_signal(&self, scrambled_signal: &Signal) -> Signal {
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
        self.0.iter().flatten().all(|x| x.is_some())
    }

}

impl std::fmt::Debug for Decoding {
    // Create an ascii table that looks something like this::
    //   0 1 2 3 4 5 6
    // 0 x . . . . ? ?
    // 1 . x . . . . .
    // 2 . . x . . . .
    // 3 . . . x . . .
    // 4 . . . . x . .
    // 5 . . . . . x .
    // 6 . . . . . . x
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}