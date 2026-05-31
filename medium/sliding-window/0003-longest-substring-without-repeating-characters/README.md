# Longest Substring Without Repeating Characters

- **Difficulty:** Medium
- **Topic:** sliding-window
- **LeetCode:** https://leetcode.com/problems/longest-substring-without-repeating-characters/

## Problem

Given a string `s`, find the length of the longest substring without repeating
characters.

## Examples

```
Input:  s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", length 3.
```
```
Input:  s = "bbbbb"
Output: 1
Explanation: The answer is "b".
```
```
Input:  s = "pwwkew"
Output: 3
Explanation: The answer is "wke". ("pwke" is a subsequence, not a substring.)
```

## Constraints

- 0 <= s.len() <= 5 * 10^4
- `s` consists of English letters, digits, symbols, and spaces.

## Function signature

```rust
pub fn length_of_longest_substring(s: String) -> i32 {
    todo!()
}
```

## Rust std hints

- Sliding window with a `HashSet<u8>` (or `HashMap<u8, usize>` of last-seen index)
  over `s.as_bytes()`.
- Grow the right edge; when a duplicate appears, shrink from the left until it's
  gone.

## Notes / approaches

Two indices bound the window. Track the max window size as you expand. Working on
`s.as_bytes()` keeps indexing simple since the input is ASCII-ish.
