use valid_anagram::is_anagram;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn is_an_anagram() {
    assert!(is_anagram(s("anagram"), s("nagaram")));
}

#[test]
fn not_an_anagram() {
    assert!(!is_anagram(s("rat"), s("car")));
}

#[test]
fn different_lengths() {
    assert!(!is_anagram(s("a"), s("ab")));
}

#[test]
fn same_letters_different_counts() {
    assert!(!is_anagram(s("aacc"), s("ccac")));
}
