# Plus One

- **Difficulty:** Easy
- **Topic:** math
- **LeetCode:** https://leetcode.com/problems/plus-one/

## Problem

You are given a large integer represented as an array of digits `digits`, where
each `digits[i]` is the `i`-th digit, most significant first. The integer has no
leading zeros. Increment it by one and return the resulting array of digits.

## Examples

```
Input:  digits = [1,2,3]
Output: [1,2,4]
```
```
Input:  digits = [9]
Output: [1,0]
Explanation: 9 + 1 = 10.
```
```
Input:  digits = [4,3,2,1]
Output: [4,3,2,2]
```

## Constraints

- 1 <= digits.len() <= 100
- 0 <= digits[i] <= 9
- `digits` has no leading zeros.

## Function signature

```rust
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    todo!()
}
```

## Rust std hints

- Walk from the last digit backward, carrying. `Iterator::rev`.
- If every digit was `9`, you need to prepend a leading `1` (`Vec::insert`).

## Notes / approaches

Handle the carry; the only length change happens when the number is all nines.
