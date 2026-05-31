# Search Insert Position

- **Difficulty:** Easy
- **Topic:** binary-search
- **LeetCode:** https://leetcode.com/problems/search-insert-position/

## Problem

Given a sorted array of distinct integers `nums` and a `target`, return the index
if the target is found. If not, return the index where it would be inserted to
keep the array sorted. Must run in `O(log n)`.

## Examples

```
Input:  nums = [1,3,5,6], target = 5
Output: 2
```
```
Input:  nums = [1,3,5,6], target = 2
Output: 1
```
```
Input:  nums = [1,3,5,6], target = 7
Output: 4
```

## Constraints

- 1 <= nums.len() <= 10^4
- -10^4 <= nums[i], target <= 10^4
- `nums` contains distinct values sorted in ascending order.

## Function signature

```rust
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}
```

## Rust std hints

- This is the "lower bound" variant of binary search: find the first index whose
  value is `>= target`.
- `slice::partition_point` does exactly this in one call if you want a shortcut.

## Notes / approaches

When the loop ends, `lo` is the insertion point — that single value answers both
the "found" and "not found" cases.
