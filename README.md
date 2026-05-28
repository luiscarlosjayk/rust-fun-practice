# LeetCode in Rust

Practicing LeetCode problems in Rust, level by level and topic by topic, to get
fluent with std types (`Vec`, `HashMap`, `HashSet`, `String`/`&str`, iterators)
and build toward Data Structures & Algorithms.

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

## Progress

### Easy

| Topic | Problem | LC # | Status |
|---|---|---|---|
| arrays-hashing | Two Sum | 1 | ☐ |
| arrays-hashing | Contains Duplicate | 217 | ☐ |
| arrays-hashing | Valid Anagram | 242 | ☐ |
| strings | Longest Common Prefix | 14 | ☐ |
| strings | Reverse String | 344 | ☐ |
| two-pointers | Valid Palindrome | 125 | ☐ |
| stack | Valid Parentheses | 20 | ☐ |
