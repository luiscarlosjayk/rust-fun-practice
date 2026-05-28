use valid_palindrome::is_palindrome;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn classic_palindrome() {
    assert!(is_palindrome(s("A man, a plan, a canal: Panama")));
}

#[test]
fn not_a_palindrome() {
    assert!(!is_palindrome(s("race a car")));
}

#[test]
fn only_punctuation_is_empty() {
    assert!(is_palindrome(s(" ")));
}

#[test]
fn alphanumeric_mismatch() {
    // filters to "0p" -> '0' != 'p'
    assert!(!is_palindrome(s("0P")));
}

#[test]
fn empty_string() {
    assert!(is_palindrome(s("")));
}
