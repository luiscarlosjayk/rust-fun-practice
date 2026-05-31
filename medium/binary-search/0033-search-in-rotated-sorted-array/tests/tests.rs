use search_in_rotated_sorted_array::search;

#[test]
fn found_in_rotated_tail() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}

#[test]
fn not_found() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}

#[test]
fn single_miss() {
    assert_eq!(search(vec![1], 0), -1);
}

#[test]
fn found_in_rotated_head() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 5), 1);
}

#[test]
fn not_rotated() {
    assert_eq!(search(vec![1, 2, 3, 4, 5], 4), 3);
    assert_eq!(search(vec![1, 2, 3, 4, 5], 6), -1);
}
