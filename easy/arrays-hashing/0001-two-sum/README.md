# Two Sum

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/two-sum/

## Problem

Given an array of integers `nums` and an integer `target`, return the indices of
the two numbers that add up to `target`. Each input has exactly one solution, and
you may not use the same element twice. The answer may be returned in any order.

## Examples

```
Input:  nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: nums[0] + nums[1] == 9
```
```
Input:  nums = [3,2,4], target = 6
Output: [1,2]
```

## Constraints

- 2 <= nums.len() <= 10^4
- -10^9 <= nums[i] <= 10^9
- Exactly one valid answer exists.

## Function signature

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}
```

## Rust std hints

- `HashMap` to remember `value -> index` as you scan once.
- `HashMap::get`, `HashMap::insert`, `Iterator::enumerate`.

## Notes / approaches

One pass: for each `n`, check if `target - n` was already seen.
