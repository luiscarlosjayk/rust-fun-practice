use climbing_stairs::climb_stairs;

#[test]
fn two_steps() {
    assert_eq!(climb_stairs(2), 2);
}

#[test]
fn three_steps() {
    assert_eq!(climb_stairs(3), 3);
}

#[test]
fn one_step() {
    assert_eq!(climb_stairs(1), 1);
}

#[test]
fn five_steps() {
    assert_eq!(climb_stairs(5), 8);
}

#[test]
fn max_input() {
    assert_eq!(climb_stairs(45), 1_836_311_903);
}
