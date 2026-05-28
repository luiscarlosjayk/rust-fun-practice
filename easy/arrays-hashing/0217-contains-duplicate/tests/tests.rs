use contains_duplicate::contains_duplicate;

#[test]
fn has_duplicate() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
}

#[test]
fn all_distinct() {
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn many_duplicates() {
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}

#[test]
fn single_element() {
    assert!(!contains_duplicate(vec![7]));
}
