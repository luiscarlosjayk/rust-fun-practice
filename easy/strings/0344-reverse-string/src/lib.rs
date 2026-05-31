//! See README.md for the problem statement.

pub fn reverse_string(s: &mut Vec<char>) {
    let s_len = s.len();
    let mut i = 0;
    while i < s_len/2 {
        s.swap(i, s_len - i - 1);
        i += 1;
    }
}
