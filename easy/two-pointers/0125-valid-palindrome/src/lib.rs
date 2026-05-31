//! See README.md for the problem statement.

pub fn is_palindrome(s: String) -> bool {
    let s_chars = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();
    let s_len = s_chars.len();
    let mut i = 0;
    let mut left: char;
    let mut right: char;

    while i < s_len / 2 {
        left = s_chars[i];
        right = s_chars[s_len - i - 1];
        if left != right {
            return false;
        }
        i += 1;
    }

    true
}
