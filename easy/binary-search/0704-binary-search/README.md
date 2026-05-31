# Binary Search

- **Difficulty:** Easy
- **Topic:** binary-search
- **LeetCode:** https://leetcode.com/problems/binary-search/

## Problem

Given a sorted (ascending) array of distinct integers `nums` and an integer
`target`, return the index of `target` in `nums`, or `-1` if it is not present.
You must write an algorithm with `O(log n)` runtime complexity.

## Examples

```
Input:  nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: nums[4] == 9
```
```
Input:  nums = [-1,0,3,5,9,12], target = 2
Output: -1
```

## Constraints

- 1 <= nums.len() <= 10^4
- -10^4 < nums[i], target < 10^4
- All integers in `nums` are unique and sorted in ascending order.

## Function signature

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}
```

## Rust std hints

- Use `usize` for the `lo`/`hi` indices and compute the midpoint as
  `lo + (hi - lo) / 2` to avoid overflow.
- Return type is `i32`, so cast the found index with `as i32`.

## Notes / approaches

Classic two-pointer binary search. Watch the loop bound (`lo <= hi` vs `lo < hi`)
and how you move past the midpoint.
