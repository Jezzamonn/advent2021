use std::{collections::HashMap, str::Lines};

use crate::pairs::Pair;

#[derive(Debug)]
pub struct Rules(HashMap<Pair, char>);

impl Rules {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Parses a string like "CH -> B" and adds a rule to the ruleset.
    pub fn add_rule(&mut self, s: &str) {
        // Let's just assume the arrow is in the right place and just read out the characters.
        self.0.insert(
            (s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()),
            s.chars().nth(6).unwrap(),
        );
    }

    pub fn from_lines(lines: &mut Lines) -> Self {
        let mut rules = Self::new();
        for l in lines {
            rules.add_rule(l);
        }
        rules
    }

    pub fn apply_rules_to_str(&self, s: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        // Loop through each pair of characters
        for i in 0..chars.len() - 1 {
            let c1 = chars[i];
            let c2 = chars[i + 1];

            result.push(c1);
            // Maybe not a rule for every pair?
            if let Some(to_insert) = self.0.get(&(c1, c2)) {
                result.push(*to_insert);
            }
        }
        // Add the last character
        result.push(chars[chars.len() - 1]);

        result
    }

    // pub fn apply_rules_to_pairs(&self, pairs: &HashMap<Pair, u32>) -> &HashMap<Pair, u32> {
    //     let mut result = String::new();
    //     let chars: Vec<char> = s.chars().collect();
    //     // Loop through each pair of characters
    //     for i in 0..chars.len() - 1 {
    //         let c1 = chars[i];
    //         let c2 = chars[i + 1];

    //         result.push(c1);
    //         // Maybe not a rule for every pair?
    //         if let Some(to_insert) = self.0.get(&(c1, c2)) {
    //             result.push(*to_insert);
    //         }
    //     }
    //     // Add the last character
    //     result.push(chars[chars.len() - 1]);

    //     result
    // }
}
