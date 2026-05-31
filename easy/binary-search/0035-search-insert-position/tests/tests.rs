use search_insert_position::search_insert;

#[test]
fn found() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn insert_middle() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
}

#[test]
fn insert_end() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn insert_front() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
}

#[test]
fn single() {
    assert_eq!(search_insert(vec![1], 0), 0);
    assert_eq!(search_insert(vec![1], 2), 1);
}
