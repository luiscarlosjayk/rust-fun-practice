# Edit Distance

- **Difficulty:** Hard
- **Topic:** dynamic-programming
- **LeetCode:** https://leetcode.com/problems/edit-distance/

## Problem

Given two strings `word1` and `word2`, return the minimum number of single-
character operations needed to turn `word1` into `word2`. The allowed operations
are insert a character, delete a character, and replace a character (the
Levenshtein distance).

## Examples

```
Input:  word1 = "horse", word2 = "ros"
Output: 3
Explanation: horse -> rorse (replace 'h') -> rose (delete 'r') -> ros (delete 'e').
```
```
Input:  word1 = "intention", word2 = "execution"
Output: 5
```

## Constraints

- 0 <= word1.len(), word2.len() <= 500
- `word1` and `word2` consist of lowercase English letters.

## Function signature

```rust
pub fn min_distance(word1: String, word2: String) -> i32 {
    todo!()
}
```

## Rust std hints

- Classic 2-D DP: `dp[i][j]` = edit distance between the first `i` chars of
  `word1` and first `j` of `word2`. Work over `word1.as_bytes()` /
  `word2.as_bytes()`.
- Base rows/cols are `0..=len` (turning a prefix into the empty string).
- You can compress to a single rolling row (`Vec<i32>` of length `n + 1`) for
  `O(n)` space.

## Notes / approaches

If the current characters match, `dp[i][j] = dp[i-1][j-1]`. Otherwise it's
`1 + min(delete = dp[i-1][j], insert = dp[i][j-1], replace = dp[i-1][j-1])`.
