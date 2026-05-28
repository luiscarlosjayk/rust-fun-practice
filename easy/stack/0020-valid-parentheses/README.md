# Valid Parentheses

- **Difficulty:** Easy
- **Topic:** stack
- **LeetCode:** https://leetcode.com/problems/valid-parentheses/

## Problem

Given a string `s` containing just the characters `()[]{}`, determine if the
input is valid. Brackets must close in the correct order and every closing
bracket must match the most recent unclosed opening bracket of the same type.

## Examples

```
Input:  s = "()[]{}"
Output: true
```
```
Input:  s = "(]"
Output: false
```
```
Input:  s = "([{}])"
Output: true
```

## Constraints

- 1 <= s.len() <= 10^4
- `s` consists only of the characters `()[]{}`.

## Function signature

```rust
pub fn is_valid(s: String) -> bool {
    todo!()
}
```

## Rust std hints

- `Vec` as a stack: `push` / `pop` (`pop` returns `Option`).
- `HashMap::from([...])` to map each closing bracket to its opener.

## Notes / approaches

Push openers; on a closer, pop and check it matches. Valid iff the stack is
empty at the end.
