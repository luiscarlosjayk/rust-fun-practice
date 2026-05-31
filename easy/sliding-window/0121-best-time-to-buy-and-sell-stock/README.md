# Best Time to Buy and Sell Stock

- **Difficulty:** Easy
- **Topic:** sliding-window
- **LeetCode:** https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

## Problem

You are given an array `prices` where `prices[i]` is the price of a stock on day
`i`. Choose a single day to buy and a later day to sell. Return the maximum
profit you can achieve. If no profit is possible, return `0`.

## Examples

```
Input:  prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1), sell on day 5 (price = 6); profit = 5.
```
```
Input:  prices = [7,6,4,3,1]
Output: 0
Explanation: Prices only fall, so no profitable transaction exists.
```

## Constraints

- 1 <= prices.len() <= 10^5
- 0 <= prices[i] <= 10^4

## Function signature

```rust
pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!()
}
```

## Rust std hints

- Track the minimum price seen so far while scanning left to right.
- `i32::min` / `i32::max` keep the running minimum and best profit.

## Notes / approaches

Single pass: at each price, the best sell-today profit is `price - min_so_far`.
Keep the max of those. This is the "expanding window" view — the window's left
edge is the cheapest buy day seen so far.
