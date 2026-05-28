# Valid Palindrome

- **Difficulty:** Easy
- **Topic:** two-pointers
- **LeetCode:** https://leetcode.com/problems/valid-palindrome/

## Problem

A phrase is a palindrome if, after lowercasing and removing all non-alphanumeric
characters, it reads the same forward and backward. Given a string `s`, return
`true` if it is a palindrome.

## Examples

```
Input:  s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" reads the same both ways.
```
```
Input:  s = "race a car"
Output: false
```
```
Input:  s = " "
Output: true
Explanation: After removing non-alphanumerics it is empty, which is a palindrome.
```

## Constraints

- 1 <= s.len() <= 2 * 10^5
- `s` consists of printable ASCII characters.

## Function signature

```rust
pub fn is_palindrome(s: String) -> bool {
    todo!()
}
```

## Rust std hints

- `char::is_alphanumeric`, `char::to_ascii_lowercase`.
- `Iterator::filter` + `Iterator::map` + `collect::<Vec<char>>()`.

## Notes / approaches

Normalize to a `Vec<char>` of lowercase alphanumerics, then two-pointer compare
from both ends.
