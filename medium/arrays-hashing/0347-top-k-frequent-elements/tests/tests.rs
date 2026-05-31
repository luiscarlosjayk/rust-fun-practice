use top_k_frequent_elements::top_k_frequent;

/// The answer set is unique but its order is not specified, so sort before
/// comparing.
fn check(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
    let mut got = top_k_frequent(nums, k);
    got.sort();
    let mut exp = expected;
    exp.sort();
    assert_eq!(got, exp);
}

#[test]
fn example_1() {
    check(vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]);
}

#[test]
fn single() {
    check(vec![1], 1, vec![1]);
}

#[test]
fn all_distinct() {
    check(vec![4, 5, 6], 3, vec![4, 5, 6]);
}

#[test]
fn with_negatives() {
    check(vec![-1, -1, -1, 2, 2, 100], 2, vec![-1, 2]);
}
