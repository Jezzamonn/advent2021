use crate::decoding::Decoding;
use crate::signal::Signal;

#[derive(Debug)]
pub struct Scenario {
    pub reading: Vec<Signal>,
    decoding: Decoding,
}

impl Scenario {
    pub fn from_line(line: &str) -> Option<Self> {
        if line.is_empty() {
            return Option::None;
        }

        let parts: Vec<&str> = line.split(" | ").collect();

        if parts.len() != 2 {
            panic!("Invalid line: {}", line);
        }

        let sample_signals: Vec<Signal> = parts[0]
            .split(' ')
            .map(Signal::from_string)
            .collect();

        let decoding = Decoding::from_sample_signals(&sample_signals);

        Option::Some(Self {
            reading: parts[1]
                .split(' ')
                .map(Signal::from_string)
                .collect(),
            decoding,
        })
    }

    fn decoded_reading(&self) -> Vec<Signal> {
        self.reading
            .iter()
            .map(|s| self.decoding.unscramble_signal(s))
            .collect()
    }

    pub fn print_debug_info(&self) {
        println!("Decoding:\n{:?}", self.decoding);
        println!("Reading:\n");
        for r in &self.reading {
            println!("{}", r.to_ascii());
        }
        println!("Decoded reading:\n");
        for r in self.decoded_reading() {
            println!("{}", r.to_ascii());
        }
    }

    pub fn reading_as_int(&self) -> i32 {
        self.decoded_reading()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, s)| s.as_digit().unwrap() * 10_i32.pow(i as u32))
            .sum()
    }
}