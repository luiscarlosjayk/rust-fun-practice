# Reverse String

- **Difficulty:** Easy
- **Topic:** strings
- **LeetCode:** https://leetcode.com/problems/reverse-string/

## Problem

Write a function that reverses a string. The input is given as a `Vec<char>` and
you must reverse it **in place** with O(1) extra memory.

## Examples

```
Input:  s = ['h','e','l','l','o']
Output: ['o','l','l','e','h']
```
```
Input:  s = ['H','a','n','n','a','h']
Output: ['h','a','n','n','a','H']
```

## Constraints

- 1 <= s.len() <= 10^5
- `s[i]` is a printable ASCII character.

## Function signature

```rust
pub fn reverse_string(s: &mut Vec<char>) {
    todo!()
}
```

## Rust std hints

- `Vec::swap(i, j)` swaps two elements without cloning.
- `Vec::len`, `usize::saturating_sub` to avoid underflow on an empty vec.

## Notes / approaches

Two pointers from both ends, swapping inward until they meet.
