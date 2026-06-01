# Median of Two Sorted Arrays

- **Difficulty:** Hard
- **Topic:** binary-search
- **LeetCode:** https://leetcode.com/problems/median-of-two-sorted-arrays/

## Problem

Given two sorted arrays `nums1` and `nums2` of sizes `m` and `n`, return the
median of the combined sorted array. The overall run time should be
`O(log(m + n))`.

## Examples

```
Input:  nums1 = [1,3], nums2 = [2]
Output: 2.0
Explanation: merged = [1,2,3], median is 2.
```
```
Input:  nums1 = [1,2], nums2 = [3,4]
Output: 2.5
Explanation: merged = [1,2,3,4], median is (2 + 3) / 2 = 2.5.
```

## Constraints

- `nums1.len() == m`, `nums2.len() == n`
- 0 <= m, n <= 1000
- 1 <= m + n <= 2000
- -10^6 <= nums1[i], nums2[i] <= 10^6

## Function signature

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    todo!()
}
```

## Rust std hints

- Binary-search a partition of the **shorter** array; derive the partition of the
  other from the total length, then compare the four boundary values.
- Use `i32::MIN` / `i32::MAX` (as `f64`) as sentinels for empty left/right sides.
- Cast to `f64` only at the end; average the two middle values for even totals.

## Notes / approaches

The `O(log(min(m, n)))` approach binary-searches where to cut the smaller array
so that everything left of both cuts is <= everything right. A simpler
`O(m + n)` merge-count is fine to get the tests green first, then optimize.
