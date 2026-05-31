# Climbing Stairs

- **Difficulty:** Easy
- **Topic:** dynamic-programming
- **LeetCode:** https://leetcode.com/problems/climbing-stairs/

## Problem

You are climbing a staircase with `n` steps. Each time you can climb either `1`
or `2` steps. In how many distinct ways can you reach the top?

## Examples

```
Input:  n = 2
Output: 2
Explanation: (1+1) and (2).
```
```
Input:  n = 3
Output: 3
Explanation: (1+1+1), (1+2), (2+1).
```

## Constraints

- 1 <= n <= 45

## Function signature

```rust
pub fn climb_stairs(n: i32) -> i32 {
    todo!()
}
```

## Rust std hints

- The answer is the Fibonacci sequence: `ways(n) = ways(n-1) + ways(n-2)`.
- Keep two rolling variables instead of an array — `O(1)` space.

## Notes / approaches

To reach step `n` your last move was either from `n-1` or `n-2`, so the counts
add. Iterate bottom-up. `n = 45` fits comfortably in `i32`.
