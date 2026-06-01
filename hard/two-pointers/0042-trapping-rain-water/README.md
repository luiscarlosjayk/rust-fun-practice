# Trapping Rain Water

- **Difficulty:** Hard
- **Topic:** two-pointers
- **LeetCode:** https://leetcode.com/problems/trapping-rain-water/

## Problem

Given `n` non-negative integers where each value is the height of a bar of width
`1`, compute how much water is trapped between the bars after it rains. Water
rests on top of a bar up to the level of the shorter of the tallest bars to its
left and right.

## Examples

```
Input:  height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The dips between taller bars hold 6 units of water in total.
```
```
Input:  height = [4,2,0,3,2,5]
Output: 9
```

## Constraints

- `n == height.len()`
- 0 <= n <= 2 * 10^4
- 0 <= height[i] <= 10^5

## Function signature

```rust
pub fn trap(height: Vec<i32>) -> i32 {
    todo!()
}
```

## Rust std hints

- Two pointers `left`/`right` walking inward, tracking `left_max` and
  `right_max`. Advance the side whose max is smaller and add `max - height[i]`.
- `i32::max`, plain index access; no allocation needed for the O(1)-space version.

## Notes / approaches

Water above bar `i` is `min(left_max[i], right_max[i]) - height[i]`. The
two-pointer trick avoids precomputing both prefix arrays: whichever side has the
smaller running max is the binding constraint, so it's safe to settle that bar.
