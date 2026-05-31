use three_sum::three_sum;

/// Triplet and list order are unspecified. Sort within each triplet and sort the
/// list of triplets before comparing.
fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for t in v.iter_mut() {
        t.sort();
    }
    v.sort();
    v
}

fn check(nums: Vec<i32>, expected: Vec<Vec<i32>>) {
    assert_eq!(normalize(three_sum(nums)), normalize(expected));
}

#[test]
fn example_1() {
    check(vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn no_triplet() {
    check(vec![0, 1, 1], vec![]);
}

#[test]
fn all_zeros() {
    check(vec![0, 0, 0], vec![vec![0, 0, 0]]);
}

#[test]
fn many_duplicates() {
    check(
        vec![-2, 0, 0, 2, 2],
        vec![vec![-2, 0, 2]],
    );
}
