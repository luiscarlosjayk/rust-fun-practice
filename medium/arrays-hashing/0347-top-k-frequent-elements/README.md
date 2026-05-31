# Top K Frequent Elements

- **Difficulty:** Medium
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/top-k-frequent-elements/

## Problem

Given an integer array `nums` and an integer `k`, return the `k` most frequent
elements. The answer may be returned in any order. It is guaranteed to be unique.

## Examples

```
Input:  nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
```
```
Input:  nums = [1], k = 1
Output: [1]
```

## Constraints

- 1 <= nums.len() <= 10^5
- -10^4 <= nums[i] <= 10^4
- 1 <= k <= number of distinct elements in `nums`
- The answer is unique.

## Function signature

```rust
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
```

## Rust std hints

- Count with a `HashMap<i32, i32>`.
- Then either sort the entries by count, or use bucket sort indexed by frequency
  for `O(n)` (`nums.len()` buckets).

## Notes / approaches

Bucket sort is the textbook `O(n)`: index a `Vec` of buckets by frequency, push
each value into `buckets[count]`, then read buckets from the high end until you
have `k` values.
