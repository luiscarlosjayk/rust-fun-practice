//! See README.md for the problem statement.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // strs has at least one string
    if strs.is_empty() {
        return String::from("");
    }
    let mut prefix = strs[0].clone();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}
