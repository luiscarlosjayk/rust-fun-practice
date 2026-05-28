use two_sum::two_sum;

// Two Sum may return the pair in any order; normalize before comparing.
fn check(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
    let mut got = two_sum(nums, target);
    got.sort();
    let mut exp = expected;
    exp.sort();
    assert_eq!(got, exp);
}

#[test]
fn example_1() {
    check(vec![2, 7, 11, 15], 9, vec![0, 1]);
}

#[test]
fn example_2() {
    check(vec![3, 2, 4], 6, vec![1, 2]);
}

#[test]
fn duplicates() {
    check(vec![3, 3], 6, vec![0, 1]);
}

#[test]
fn negatives() {
    check(vec![-3, 4, 3, 90], 0, vec![0, 2]);
}
