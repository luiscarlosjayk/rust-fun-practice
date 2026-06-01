use edit_distance::min_distance;

fn check(a: &str, b: &str, expected: i32) {
    assert_eq!(min_distance(a.to_string(), b.to_string()), expected);
}

#[test]
fn example_1() {
    check("horse", "ros", 3);
}

#[test]
fn example_2() {
    check("intention", "execution", 5);
}

#[test]
fn from_empty() {
    check("", "abc", 3);
}

#[test]
fn to_empty() {
    check("abc", "", 3);
}

#[test]
fn identical() {
    check("abc", "abc", 0);
}

#[test]
fn both_empty() {
    check("", "", 0);
}
