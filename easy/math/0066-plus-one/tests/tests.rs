use plus_one::plus_one;

#[test]
fn example_1() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
}

#[test]
fn carry_to_new_digit() {
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
}

#[test]
fn example_3() {
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
}

#[test]
fn all_nines() {
    assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
}

#[test]
fn internal_carry() {
    assert_eq!(plus_one(vec![1, 9, 9]), vec![2, 0, 0]);
}
