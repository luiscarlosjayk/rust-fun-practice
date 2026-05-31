//! See README.md for the problem statement.
use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashMap::new();

    for (_i, num) in nums.iter().enumerate() {
        if let Some(_value) = seen.get(&num) {
            return true;
        }

        seen.entry(num).or_insert_with(|| ());
    }

    false
}
