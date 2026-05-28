# Valid Anagram

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/valid-anagram/

## Problem

Given two strings `s` and `t`, return `true` if `t` is an anagram of `s` (the
same characters with the same counts, just reordered).

## Examples

```
Input:  s = "anagram", t = "nagaram"
Output: true
```
```
Input:  s = "rat", t = "car"
Output: false
```

## Constraints

- 1 <= s.len(), t.len() <= 5 * 10^4
- `s` and `t` consist of lowercase English letters.

## Function signature

```rust
pub fn is_anagram(s: String, t: String) -> bool {
    todo!()
}
```

## Rust std hints

- `HashMap<char, i32>` with `entry(c).or_insert(0)` to count.
- `String::chars`, `HashMap::values`.

## Notes / approaches

Count chars in `s` (increment) and `t` (decrement); they match iff all counts
end at zero. Differing lengths can never be anagrams.
