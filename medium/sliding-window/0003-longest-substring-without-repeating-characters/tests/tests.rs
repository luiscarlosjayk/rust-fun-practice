use longest_substring_without_repeating_characters::length_of_longest_substring;

fn check(s: &str, expected: i32) {
    assert_eq!(length_of_longest_substring(s.to_string()), expected);
}

#[test]
fn example_1() {
    check("abcabcbb", 3);
}

#[test]
fn all_same() {
    check("bbbbb", 1);
}

#[test]
fn example_3() {
    check("pwwkew", 3);
}

#[test]
fn empty() {
    check("", 0);
}

#[test]
fn with_spaces() {
    check(" ", 1);
    check("dvdf", 3);
}
