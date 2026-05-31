# Search in Rotated Sorted Array

- **Difficulty:** Medium
- **Topic:** binary-search
- **LeetCode:** https://leetcode.com/problems/search-in-rotated-sorted-array/

## Problem

An ascending array of distinct integers was rotated at some unknown pivot (so
`[0,1,2,4,5,6,7]` might become `[4,5,6,7,0,1,2]`). Given the rotated array `nums`
and a `target`, return its index, or `-1` if absent. Must run in `O(log n)`.

## Examples

```
Input:  nums = [4,5,6,7,0,1,2], target = 0
Output: 4
```
```
Input:  nums = [4,5,6,7,0,1,2], target = 3
Output: -1
```
```
Input:  nums = [1], target = 0
Output: -1
```

## Constraints

- 1 <= nums.len() <= 5000
- -10^4 <= nums[i], target <= 10^4
- All values of `nums` are unique; `nums` is a rotated ascending array.

## Function signature

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}
```

## Rust std hints

- Still one binary search. At each midpoint, one half is guaranteed sorted —
  decide which, then check whether `target` falls inside that sorted half.

## Notes / approaches

Compare `nums[mid]` with `nums[lo]` to learn which side is sorted. If `target` is
within the sorted side's range, search there; otherwise search the other side.
