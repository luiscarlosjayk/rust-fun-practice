# 3Sum

- **Difficulty:** Medium
- **Topic:** two-pointers
- **LeetCode:** https://leetcode.com/problems/3sum/

## Problem

Given an integer array `nums`, return all triplets `[nums[i], nums[j], nums[k]]`
such that `i`, `j`, `k` are distinct indices and `nums[i] + nums[j] + nums[k] == 0`.
The solution set must not contain duplicate triplets. Triplets and the overall
list may be returned in any order.

## Examples

```
Input:  nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
```
```
Input:  nums = [0,1,1]
Output: []
```
```
Input:  nums = [0,0,0]
Output: [[0,0,0]]
```

## Constraints

- 3 <= nums.len() <= 3000
- -10^5 <= nums[i] <= 10^5

## Function signature

```rust
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
```

## Rust std hints

- Sort first (`slice::sort`). Then fix one index and use two pointers moving
  inward on the rest.
- Skip equal neighbors to avoid duplicate triplets.

## Notes / approaches

`O(n^2)`: for each `i`, find pairs in `nums[i+1..]` summing to `-nums[i]` with the
two-pointer sweep. Sorting makes both the search and the de-duplication easy.
