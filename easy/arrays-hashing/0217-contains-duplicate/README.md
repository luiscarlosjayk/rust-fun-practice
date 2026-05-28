# Contains Duplicate

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/contains-duplicate/

## Problem

Given an integer array `nums`, return `true` if any value appears at least twice,
and `false` if every element is distinct.

## Examples

```
Input:  nums = [1,2,3,1]
Output: true
```
```
Input:  nums = [1,2,3,4]
Output: false
```

## Constraints

- 1 <= nums.len() <= 10^5
- -10^9 <= nums[i] <= 10^9

## Function signature

```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}
```

## Rust std hints

- `HashSet::insert` returns `false` when the value was already present.

## Notes / approaches

Insert each value into a `HashSet`; if an insert fails, you found a duplicate.
