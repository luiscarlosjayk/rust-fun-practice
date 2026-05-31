# Group Anagrams

- **Difficulty:** Medium
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/group-anagrams/

## Problem

Given an array of strings `strs`, group the anagrams together. Two strings are
anagrams if one is a rearrangement of the other's letters. Return the groups in
any order; the strings within each group may be in any order too.

## Examples

```
Input:  strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
```
```
Input:  strs = [""]
Output: [[""]]
```
```
Input:  strs = ["a"]
Output: [["a"]]
```

## Constraints

- 1 <= strs.len() <= 10^4
- 0 <= strs[i].len() <= 100
- `strs[i]` consists of lowercase English letters.

## Function signature

```rust
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    todo!()
}
```

## Rust std hints

- Build a canonical key per word. Either sort its bytes, or use a length-26 count
  array as the key.
- `HashMap<Key, Vec<String>>` then collect the values.

## Notes / approaches

Anagrams share a key. The output order is unspecified, so just return
`map.into_values().collect()`.
