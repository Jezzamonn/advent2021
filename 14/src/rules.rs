use std::collections::HashMap;

pub type Rules = HashMap<(char, char), char>;

/// Parses a string like "CH -> B" and adds a rule to the ruleset.
pub fn add_rule(rules: &mut Rules, s: &str) {
    // Let's just assume the arrow is in the right place and just read out the characters.
    rules.insert(
        (s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()),
        s.chars().nth(6).unwrap(),
    );
}

pub fn apply_rules(s: &str, rules: &Rules) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    // Loop through each pair of characters
    for i in 0..chars.len() - 1 {
        let c1 = chars[i];
        let c2 = chars[i + 1];

        result.push(c1);
        // Maybe not a rule for every pair?
        if let Some(to_insert) = rules.get(&(c1, c2)) {
            result.push(*to_insert);
        }
    }
    // Add the last character
    result.push(chars[chars.len() - 1]);

    result
}
