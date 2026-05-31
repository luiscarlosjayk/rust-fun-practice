use remove_duplicates_from_sorted_array::remove_duplicates;

/// Runs the function and checks both the returned length and the first `k`
/// elements (the order LeetCode validates).
fn check(mut nums: Vec<i32>, expected_prefix: Vec<i32>) {
    let k = remove_duplicates(&mut nums);
    assert_eq!(k as usize, expected_prefix.len(), "wrong unique count");
    assert_eq!(&nums[..expected_prefix.len()], &expected_prefix[..]);
}

#[test]
fn example_1() {
    check(vec![1, 1, 2], vec![1, 2]);
}

#[test]
fn example_2() {
    check(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]);
}

#[test]
fn single() {
    check(vec![7], vec![7]);
}

#[test]
fn all_same() {
    check(vec![2, 2, 2, 2], vec![2]);
}

#[test]
fn no_duplicates() {
    check(vec![-3, -1, 0, 5], vec![-3, -1, 0, 5]);
}
