use minimum_window_substring::min_window;

fn check(s: &str, t: &str, expected: &str) {
    assert_eq!(min_window(s.to_string(), t.to_string()), expected);
}

#[test]
fn example_1() {
    check("ADOBECODEBANC", "ABC", "BANC");
}

#[test]
fn single_match() {
    check("a", "a", "a");
}

#[test]
fn not_enough_copies() {
    check("a", "aa", "");
}

#[test]
fn no_window() {
    check("a", "b", "");
}

#[test]
fn duplicates_in_target() {
    check("aa", "aa", "aa");
}

#[test]
fn whole_string_is_window() {
    check("ab", "ba", "ab");
}
