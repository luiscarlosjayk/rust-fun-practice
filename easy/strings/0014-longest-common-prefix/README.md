# Longest Common Prefix

- **Difficulty:** Easy
- **Topic:** strings
- **LeetCode:** https://leetcode.com/problems/longest-common-prefix/

## Problem

Write a function to find the longest common prefix string amongst an array of
strings. If there is no common prefix, return the empty string `""`.

## Examples

```
Input:  strs = ["flower","flow","flight"]
Output: "fl"
```
```
Input:  strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
```

## Constraints

- 1 <= strs.len() <= 200
- 0 <= strs[i].len() <= 200
- `strs[i]` consists of lowercase English letters.

## Function signature

```rust
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    todo!()
}
```

## Rust std hints

- `str::starts_with`, `String::pop`, `String::is_empty`.
- Slicing a `Vec` with `&strs[1..]` to iterate the rest.

## Notes / approaches

Start with the first string as the candidate prefix; for each other string,
shrink the prefix from the end until it is a prefix of that string.
