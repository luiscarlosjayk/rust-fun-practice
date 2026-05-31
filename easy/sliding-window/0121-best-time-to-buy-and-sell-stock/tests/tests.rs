use best_time_to_buy_and_sell_stock::max_profit;

#[test]
fn example_1() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn no_profit() {
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn single_day() {
    assert_eq!(max_profit(vec![5]), 0);
}

#[test]
fn increasing() {
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn dip_then_peak() {
    assert_eq!(max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
}
