use std::collections::HashMap;

pub type Pair = (char, char);

pub struct Pairs(
    pub HashMap<Pair, u32>,
);

impl Pairs {
    pub fn from_string(s: &str) -> Self {
        let mut counts = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        // Loop through each pair of characters
        for i in 0..chars.len() - 1 {
            let c1 = chars[i];
            let c2 = chars[i + 1];

            *counts.entry((c1, c2)).or_insert(0) += 1;
        }

        Self(counts)
    }
}
