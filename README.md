# Rust Fun Practice

Practicing LeetCode-style problems in Rust, level by level and topic by topic, to
get fluent with std types (`Vec`, `HashMap`, `HashSet`, `String`/`&str`,
iterators) and build toward Data Structures & Algorithms.

## Layout

Each problem is an independent crate under `<level>/<topic>/<NNNN-slug>/`:
- `README.md` — the problem statement (see `TEMPLATE.md` for the format).
- `src/lib.rs` — your solution. Starts as a `todo!()` stub; this is what you fill in.
- `tests/tests.rs` — provided test cases that validate your solution.

## Workflow

1. Pick a problem and read its `README.md`.
2. Implement the public function in `src/lib.rs`.
3. Run the tests until green:

```sh
cargo test -p <slug>        # one problem, e.g. cargo test -p two-sum
cargo test                  # every problem in the workspace
```

A fresh stub fails with a `not yet implemented` panic — that is expected until
you solve it.

## Adding problems

```sh
./new-problem.sh <level> <topic> <NNNN-slug> <fn_name>
# e.g. ./new-problem.sh easy arrays-hashing 0026-remove-duplicates remove_duplicates
```

Then fill in the generated `README.md`, `src/lib.rs`, and `tests/tests.rs`.

## Attribution

These problems are practice exercises sourced from third parties; this repo holds
only my own Rust solutions and tests.

- **Problem origin:** Each problem's `README.md` links to its canonical source
  (the `LeetCode:` line). Problem titles and numbers follow [LeetCode](https://leetcode.com).
- **Selection / ordering:** Topic grouping and the choice of which problems to
  practice are inspired by the [NeetCode 150](https://neetcode.io/practice)
  roadmap and [Hello Interview](https://www.hellointerview.com/learn/code).
- **Wording:** Problem statements are **restated in my own words**; original
  prompts, full test suites, and editorials remain the property of their
  respective owners and are not reproduced here. Follow the linked source for the
  official statement.
- **Solutions and the test cases in this repo are my own work**, dual-licensed
  under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

## Progress

### Easy

| Topic | Problem | LC # | Status |
|---|---|---|---|
| arrays-hashing | Two Sum | 1 | ☐ |
| arrays-hashing | Contains Duplicate | 217 | ☐ |
| arrays-hashing | Valid Anagram | 242 | ☐ |
| arrays-hashing | Remove Duplicates from Sorted Array | 26 | ☐ |
| strings | Longest Common Prefix | 14 | ☐ |
| strings | Reverse String | 344 | ☐ |
| two-pointers | Valid Palindrome | 125 | ☐ |
| stack | Valid Parentheses | 20 | ☐ |
| binary-search | Binary Search | 704 | ☐ |
| binary-search | Search Insert Position | 35 | ☐ |
| sliding-window | Best Time to Buy and Sell Stock | 121 | ☐ |
| linked-list | Reverse Linked List | 206 | ☐ |
| linked-list | Merge Two Sorted Lists | 21 | ☐ |
| trees | Invert Binary Tree | 226 | ☐ |
| trees | Maximum Depth of Binary Tree | 104 | ☐ |
| dynamic-programming | Climbing Stairs | 70 | ☐ |
| math | Plus One | 66 | ☐ |

### Medium

| Topic | Problem | LC # | Status |
|---|---|---|---|
| arrays-hashing | Group Anagrams | 49 | ☐ |
| arrays-hashing | Top K Frequent Elements | 347 | ☐ |
| two-pointers | 3Sum | 15 | ☐ |
| sliding-window | Longest Substring Without Repeating Characters | 3 | ☐ |
| binary-search | Search in Rotated Sorted Array | 33 | ☐ |
