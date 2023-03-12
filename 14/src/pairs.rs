use std::collections::HashMap;

pub type Pair = (char, char);

pub struct Pairs {
    pub pairs: HashMap<Pair, u64>,
    pub first: char,
    pub last: char,
}

impl Pairs {
    pub fn new(first: char, last: char) -> Self {
        Self {
            pairs: HashMap::new(),
            first,
            last,
        }
    }

    pub fn from_string(s: &str) -> Self {
        let mut counts = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        // Loop through each pair of characters
        for i in 0..chars.len() - 1 {
            let c1 = chars[i];
            let c2 = chars[i + 1];

            *counts.entry((c1, c2)).or_insert(0) += 1;
        }

        Self {
            pairs: counts,
            first: chars[0],
            last: *chars.last().unwrap(),
        }
    }

    pub fn count_chars(&self) -> HashMap<char, u64> {
        // Because each character except the first and last are in two pairs,
        // this will be double the actual count while we're creating it.
        let mut counts = HashMap::new();
        // Add the under-counted first and last
        counts.insert(self.first, 1);
        counts.insert(self.last, 1);

        for (pair, count) in &self.pairs {
            *counts.entry(pair.0).or_insert(0) += count;
            *counts.entry(pair.1).or_insert(0) += count;
        }

        // Correct the double counting
        counts.values_mut().for_each(|x| *x /= 2);

        counts
    }
}
