//! See README.md for the problem statement.
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    let mut response_indexes: Vec<i32> = Vec::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(addend_index_found) = seen.get(&(target - num)) {
            response_indexes.push(*addend_index_found);
            response_indexes.push(i as i32);
            break;
        }
        seen.entry(num).or_insert_with(|| i as i32);
    }

    response_indexes
}
