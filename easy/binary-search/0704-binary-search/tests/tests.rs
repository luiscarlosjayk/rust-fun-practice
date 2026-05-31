use binary_search::search;

#[test]
fn found_middle() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
}

#[test]
fn not_found() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}

#[test]
fn single_hit() {
    assert_eq!(search(vec![5], 5), 0);
}

#[test]
fn single_miss() {
    assert_eq!(search(vec![5], -5), -1);
}

#[test]
fn first_and_last() {
    assert_eq!(search(vec![2, 4, 6, 8, 10], 2), 0);
    assert_eq!(search(vec![2, 4, 6, 8, 10], 10), 4);
}
