//! See README.md for the problem statement.
use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut chars_count = HashMap::new();

    for char in s.chars() {
        if let Some(count) = chars_count.get(&char) {
            if *count != -1 {
                chars_count.remove(&char);
            } else {
                chars_count.insert(char, count + 1);
            }
        } else {
            chars_count.entry(char).or_insert_with(|| 1);
        }
    }
    for char in t.chars() {
        if let Some(count) = chars_count.get(&char) {
            if *count == 1 {
                chars_count.remove(&char);
            } else {
                chars_count.insert(char, count - 1);
            }
        } else {
            chars_count.entry(char).or_insert_with(|| 1);
        }
    }

    chars_count.len() == 0
}
