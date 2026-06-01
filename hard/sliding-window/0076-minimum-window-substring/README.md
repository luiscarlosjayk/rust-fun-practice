# Minimum Window Substring

- **Difficulty:** Hard
- **Topic:** sliding-window
- **LeetCode:** https://leetcode.com/problems/minimum-window-substring/

## Problem

Given strings `s` and `t`, return the shortest substring of `s` that contains
every character of `t` including duplicates. If no such window exists, return the
empty string. The answer is guaranteed to be unique when it exists.

## Examples

```
Input:  s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
Explanation: "BANC" is the shortest window containing A, B, and C.
```
```
Input:  s = "a", t = "a"
Output: "a"
```
```
Input:  s = "a", t = "aa"
Output: ""
Explanation: "a" has only one 'a', so no window covers both copies in t.
```

## Constraints

- 1 <= s.len(), t.len() <= 10^5
- `s` and `t` consist of uppercase and lowercase English letters.

## Function signature

```rust
pub fn min_window(s: String, t: String) -> String {
    todo!()
}
```

## Rust std hints

- Count `t`'s characters in a `HashMap<u8, i32>` (or `[i32; 128]`), then expand a
  right pointer and contract a left pointer over `s.as_bytes()`.
- Track a `have`/`need` counter so each step is O(1); record the best `(len,
  start)` whenever the window is valid.
- Slice the winner back out with `&s[start..start + len]`.

## Notes / approaches

Grow the window until it covers `t`, then shrink from the left as far as it stays
valid, updating the best answer. Each byte enters and leaves the window at most
once, so it's O(|s|).
