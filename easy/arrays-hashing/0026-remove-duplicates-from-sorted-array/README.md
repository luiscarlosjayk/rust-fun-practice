# Remove Duplicates from Sorted Array

- **Difficulty:** Easy
- **Topic:** arrays-hashing (two-pointers technique)
- **LeetCode:** https://leetcode.com/problems/remove-duplicates-from-sorted-array/

## Problem

Given a sorted array `nums` in non-decreasing order, remove the duplicates
**in place** so each unique element appears once, keeping the relative order. Return
`k`, the number of unique elements. The first `k` slots of `nums` must hold those
unique elements (the rest may be anything).

## Examples

```
Input:  nums = [1,1,2]
Output: 2, nums = [1,2,_]
```
```
Input:  nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
```

## Constraints

- 1 <= nums.len() <= 3 * 10^4
- -100 <= nums[i] <= 100
- `nums` is sorted in non-decreasing order.

## Function signature

```rust
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    todo!()
}
```

## Rust std hints

- Two pointers over the same slice: a slow `write` index and a fast `read` index.
- You only need to mutate in place; no extra allocation.

## Notes / approaches

Keep a write cursor at the last unique value. Advance the read cursor; whenever it
finds a new value, write it just after the cursor. Return `write + 1`.
