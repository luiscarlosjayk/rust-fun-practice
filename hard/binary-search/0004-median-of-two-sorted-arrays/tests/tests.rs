use median_of_two_sorted_arrays::find_median_sorted_arrays;

/// Medians are floats; compare within a small tolerance.
fn check(a: Vec<i32>, b: Vec<i32>, expected: f64) {
    let got = find_median_sorted_arrays(a, b);
    assert!(
        (got - expected).abs() < 1e-9,
        "got {got}, expected {expected}"
    );
}

#[test]
fn odd_total() {
    check(vec![1, 3], vec![2], 2.0);
}

#[test]
fn even_total() {
    check(vec![1, 2], vec![3, 4], 2.5);
}

#[test]
fn one_empty() {
    check(vec![], vec![1], 1.0);
    check(vec![2], vec![], 2.0);
}

#[test]
fn all_equal() {
    check(vec![0, 0], vec![0, 0], 0.0);
}

#[test]
fn interleaved() {
    check(vec![1, 3, 8, 9, 15], vec![7, 11, 18, 19, 21, 25], 11.0);
}
