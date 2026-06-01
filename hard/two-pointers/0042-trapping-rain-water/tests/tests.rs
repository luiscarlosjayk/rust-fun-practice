use trapping_rain_water::trap;

#[test]
fn example_1() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn example_2() {
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

#[test]
fn empty() {
    assert_eq!(trap(vec![]), 0);
}

#[test]
fn monotonic_holds_nothing() {
    assert_eq!(trap(vec![1, 2, 3, 4, 5]), 0);
    assert_eq!(trap(vec![5, 4, 3, 2, 1]), 0);
}

#[test]
fn single_basin() {
    assert_eq!(trap(vec![3, 0, 3]), 3);
}
