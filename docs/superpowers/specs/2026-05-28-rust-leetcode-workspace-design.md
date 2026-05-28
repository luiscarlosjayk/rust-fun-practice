# Rust LeetCode Practice Workspace — Design

**Date:** 2026-05-28
**Status:** Approved

## Goal

A structured environment to practice LeetCode problems in Rust, progressing
from easy problems toward Data Structures & Algorithms. The early focus is on
getting comfortable with Rust standard-library types (`Vec`, `HashMap`,
`HashSet`, `String`/`&str`, iterators). Problems are grouped LeetCode-style by
difficulty level and topic. The set starts small and grows over time.

## Layout — single Cargo workspace, level → topic → problem

Each problem is an independent crate, unified under one workspace so a single
`cargo test` from the root runs everything (and `cargo test -p <slug>` runs one).
Shared `target/` means dependencies compile once.

```
leetcode/
├── Cargo.toml              # workspace manifest; members list grows over time
├── README.md               # overview, how-to-use, progress tracker table
├── TEMPLATE.md             # the spec/recipe for constructing each problem
├── new-problem.sh          # scaffold script: copies template → new crate
├── docs/superpowers/specs/ # design docs (this file)
└── easy/
    ├── arrays-hashing/
    │   └── 0001-two-sum/
    │       ├── Cargo.toml   # [package] name = "two-sum"
    │       ├── README.md    # problem statement (follows TEMPLATE.md)
    │       ├── src/lib.rs    # pub fn two_sum(...) { todo!() }  ← user solves
    │       └── tests/tests.rs # provided validating test cases ← maintainer writes
    ├── strings/
    ├── two-pointers/
    └── stack/
```

### Conventions

- **Directory names** are number-prefixed (`0001-two-sum`) so problems sort
  naturally within a topic folder. The number is the LeetCode problem number,
  zero-padded to 4 digits.
- **Package names** are the clean slug (`two-sum`); the lib target is the slug
  with hyphens replaced by underscores (`two_sum`). Slugs are unique per
  LeetCode problem, so no workspace name collisions.
- **Edition** 2021, workspace `resolver = "2"`.
- **Future levels** (`medium/`, `hard/`) and topics (`linked-list/`, `trees/`,
  `dynamic-programming/`, …) follow the same `level/topic/NNNN-slug/` shape.

### Solution / validator separation

- The **solution** lives in `src/lib.rs` and starts as a `todo!()` stub with the
  exact public function signature LeetCode expects.
- The **validating tests** live in `tests/tests.rs` (integration tests). They
  call only the crate's public function, so:
  - the public signature is forced to match LeetCode's,
  - the tests remain untouched while the user iterates on the solution.

## The per-problem README (the "template")

`TEMPLATE.md` documents the recipe; every problem `README.md` follows it. Sections:

1. **Title / Difficulty / Topic / LeetCode link**
2. **Problem statement** — restated plainly, plus the original wording.
3. **Examples** — Input → Output → Explanation.
4. **Constraints** — input bounds and guarantees.
5. **Function signature** — the exact `pub fn` to implement.
6. **Rust std hints** — which std types/methods the problem exercises
   (`HashMap`, `HashSet`, `Vec`-as-stack, `chars()`, `bytes()`, etc.). This
   directly serves the "get used to std types" goal.
7. **Notes / approaches** — scratch space for the user's own reasoning.

## Starting batch — 7 problems across 4 topics

| Topic | Problem | LC # | Public fn | std focus |
|---|---|---|---|---|
| arrays-hashing | Two Sum | 1 | `two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>` | `HashMap` |
| arrays-hashing | Contains Duplicate | 217 | `contains_duplicate(nums: Vec<i32>) -> bool` | `HashSet` |
| arrays-hashing | Valid Anagram | 242 | `is_anagram(s: String, t: String) -> bool` | `HashMap` / sorting |
| strings | Reverse String | 344 | `reverse_string(s: &mut Vec<char>)` | `&mut Vec<char>`, in-place |
| strings | Longest Common Prefix | 14 | `longest_common_prefix(strs: Vec<String>) -> String` | `&str`, `bytes()`/`chars()` |
| two-pointers | Valid Palindrome | 125 | `is_palindrome(s: String) -> bool` | two-pointer, char filtering |
| stack | Valid Parentheses | 20 | `is_valid(s: String) -> bool` | `Vec` as stack + `HashMap` pairs |

Function signatures mirror LeetCode's official Rust stubs so solutions transfer
directly to the site.

## Workflow

- **Solve:** read the problem `README.md` → implement in `src/lib.rs` →
  `cargo test -p <slug>` until green.
- **Add more:** `./new-problem.sh <level> <topic> <NNNN-slug> <fn_name>` scaffolds
  a new crate from the template and appends it to the workspace members; then
  fill in the `README.md` and `tests/tests.rs`.
- The source article (Top 50 Easy LeetCode Problems) provides ~50 problems across
  further topics (linked lists, math, trees, DP, binary search) to draw from when
  expanding.

## Testing strategy

- Tests are standard `#[test]` functions in `tests/tests.rs`, using `assert_eq!`.
- Each problem ships with: the LeetCode-provided examples, plus edge cases
  (empty input, single element, all-equal, ordering quirks) chosen per problem.
- For order-insensitive results (e.g. Two Sum index pairs), tests normalize
  before comparing so any valid answer passes.
- `cargo test` (root) runs the whole suite; CI-friendly with zero config.

## Out of scope (YAGNI)

- No external crates / no benchmarking harness initially.
- No automated LeetCode submission or scraping.
- No medium/hard problems in the first batch (structure supports them later).
