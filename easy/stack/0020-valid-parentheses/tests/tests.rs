use valid_parentheses::is_valid;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn simple_pair() {
    assert!(is_valid(s("()")));
}

#[test]
fn all_types() {
    assert!(is_valid(s("()[]{}")));
}

#[test]
fn mismatched() {
    assert!(!is_valid(s("(]")));
}

#[test]
fn nested() {
    assert!(is_valid(s("([{}])")));
}

#[test]
fn unbalanced_open() {
    assert!(!is_valid(s("(")));
}

#[test]
fn unbalanced_close() {
    assert!(!is_valid(s(")")));
}
