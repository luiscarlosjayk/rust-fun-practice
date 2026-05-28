# Rust LeetCode Practice Workspace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Cargo workspace where each LeetCode problem is an independent crate with a `todo!()` solution stub and a ready-to-run validating test suite, seeded with 7 easy problems across 4 topics.

**Architecture:** One workspace at the repo root (`leetcode/`). Problems live under `<level>/<topic>/<NNNN-slug>/`, each a crate with `src/lib.rs` (the practice stub) and `tests/tests.rs` (integration tests calling the public function). A `TEMPLATE.md` documents the per-problem README format, and `new-problem.sh` scaffolds new crates. Each problem is verified during the build by temporarily pasting the reference solution (kept in this plan as an answer key), confirming the tests pass, then restoring the `todo!()` stub so the working tree is an exercise.

**Tech Stack:** Rust (edition 2024, rustc 1.94.1), Cargo workspace, std-only (no external crates), bash for the scaffold script.

---

## File Structure

```
leetcode/
├── Cargo.toml                      # workspace manifest + member list
├── README.md                       # overview, usage, progress tracker
├── TEMPLATE.md                     # per-problem README recipe
├── new-problem.sh                  # scaffold script
├── .gitignore                      # (already exists)
├── docs/superpowers/{specs,plans}/ # design + this plan (already exist)
└── easy/
    ├── arrays-hashing/
    │   ├── 0001-two-sum/{Cargo.toml, README.md, src/lib.rs, tests/tests.rs}
    │   ├── 0217-contains-duplicate/...
    │   └── 0242-valid-anagram/...
    ├── strings/
    │   ├── 0014-longest-common-prefix/...
    │   └── 0344-reverse-string/...
    ├── two-pointers/
    │   └── 0125-valid-palindrome/...
    └── stack/
        └── 0020-valid-parentheses/...
```

**Conventions applied throughout:**
- Problem dir: `<level>/<topic>/<NNNN-slug>/` (4-digit zero-padded LeetCode number).
- Package name in `Cargo.toml` = the slug (e.g. `two-sum`); the lib crate name is the slug with hyphens replaced by underscores (e.g. `two_sum`), which is what `tests/tests.rs` imports.
- Every problem `Cargo.toml` is:
  ```toml
  [package]
  name = "<slug>"
  version = "0.1.0"
  edition = "2024"

  [lib]
  path = "src/lib.rs"
  ```

---

## Task 1: Workspace skeleton

**Files:**
- Create: `Cargo.toml`
- Create: `README.md`
- Create: `TEMPLATE.md`
- Create: `new-problem.sh`

- [ ] **Step 1: Create the workspace manifest**

`Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "easy/arrays-hashing/0001-two-sum",
    "easy/arrays-hashing/0217-contains-duplicate",
    "easy/arrays-hashing/0242-valid-anagram",
    "easy/strings/0014-longest-common-prefix",
    "easy/strings/0344-reverse-string",
    "easy/two-pointers/0125-valid-palindrome",
    "easy/stack/0020-valid-parentheses",
    # NEW_PROBLEM_MARKER (new-problem.sh inserts members above this line)
]
```

- [ ] **Step 2: Create the root README**

`README.md`:
```markdown
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
```

- [ ] **Step 3: Create the per-problem README template**

`TEMPLATE.md`:
```markdown
# <Problem Title>

- **Difficulty:** <Easy | Medium | Hard>
- **Topic:** <topic>
- **LeetCode:** <https://leetcode.com/problems/...>

## Problem

<Restate the problem plainly in your own words, then the original wording.>

## Examples

**Example 1**
```
Input:  <input>
Output: <output>
Explanation: <why>
```

**Example 2**
```
Input:  <input>
Output: <output>
```

## Constraints

- <bound>
- <bound>

## Function signature

```rust
pub fn <fn_name>(<args>) -> <ret> {
    todo!()
}
```

## Rust std hints

- <std type / method this problem exercises, e.g. HashMap::entry>

## Notes / approaches

<Your scratch space: ideas, complexity, gotchas.>
```

- [ ] **Step 4: Create the scaffold script**

`new-problem.sh`:
```bash
#!/usr/bin/env bash
set -euo pipefail

# Usage: ./new-problem.sh <level> <topic> <NNNN-slug> <fn_name>
# Example: ./new-problem.sh easy arrays-hashing 0026-remove-duplicates remove_duplicates

if [ "$#" -ne 4 ]; then
  echo "Usage: $0 <level> <topic> <NNNN-slug> <fn_name>" >&2
  echo "Example: $0 easy arrays-hashing 0026-remove-duplicates remove_duplicates" >&2
  exit 1
fi

LEVEL="$1"; TOPIC="$2"; DIRNAME="$3"; FN="$4"
SLUG="${DIRNAME#*-}"          # strip leading NNNN-
PKG="$SLUG"
LIB="${PKG//-/_}"             # crate name uses underscores
DIR="$LEVEL/$TOPIC/$DIRNAME"
ROOT="$(cd "$(dirname "$0")" && pwd)"
TARGET="$ROOT/$DIR"

if [ -e "$TARGET" ]; then
  echo "Already exists: $DIR" >&2
  exit 1
fi

mkdir -p "$TARGET/src" "$TARGET/tests"

cat > "$TARGET/Cargo.toml" <<EOF
[package]
name = "$PKG"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
EOF

cat > "$TARGET/src/lib.rs" <<EOF
//! See README.md for the problem statement.

pub fn $FN() {
    todo!("implement $FN")
}
EOF

cat > "$TARGET/tests/tests.rs" <<EOF
// Validating tests for $PKG. Fill these in from the problem examples.
use ${LIB}::$FN;

#[test]
fn example_1() {
    todo!("add assertions");
}
EOF

cp "$ROOT/TEMPLATE.md" "$TARGET/README.md"

# Insert the new member into the workspace Cargo.toml, just above the marker.
awk -v line="    \"$DIR\"," '/NEW_PROBLEM_MARKER/{print line} {print}' \
  "$ROOT/Cargo.toml" > "$ROOT/Cargo.toml.tmp" && mv "$ROOT/Cargo.toml.tmp" "$ROOT/Cargo.toml"

echo "Created $DIR and added it to the workspace."
echo "Next: fill in README.md, src/lib.rs ($FN signature + body), and tests/tests.rs."
```

- [ ] **Step 5: Make the script executable**

Run: `chmod +x new-problem.sh`
Expected: no output, exit 0.

- [ ] **Step 6: Verify the workspace parses (members don't exist yet, so expect a clear error)**

Run: `cargo metadata --no-deps --format-version 1 >/dev/null`
Expected: FAILS with an error like `failed to load manifest ... easy/arrays-hashing/0001-two-sum/Cargo.toml` — confirms the manifest is syntactically valid and Cargo is reading the member list. (It succeeds once Task 2 adds the first crate.)

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml README.md TEMPLATE.md new-problem.sh
git commit -m "Add workspace skeleton, README, template, and scaffold script"
```

---

## How to do each problem task (Tasks 2–8)

Every problem task has the same shape. For each one:

1. Create `Cargo.toml` (the standard block, with the slug as `name`).
2. Create `README.md` (full statement — given below per problem).
3. Create `tests/tests.rs` (given below per problem).
4. Create `src/lib.rs` **with the reference solution** (given below) — temporarily, to validate the tests.
5. Run `cargo test -p <slug>` → expect **PASS**. This proves the tests are correct.
6. Replace the solution body with the `todo!()` stub (shown per problem).
7. Run `cargo test -p <slug>` → expect **FAIL** (panic: `not yet implemented`/`not implemented`). This confirms the practice stub is in place.
8. Commit.

The reference solution is the answer key and must NOT remain in the committed `src/lib.rs` — the committed file is the stub from step 6.

---

## Task 2: Two Sum (easy / arrays-hashing / 0001-two-sum)

**Files:**
- Create: `easy/arrays-hashing/0001-two-sum/Cargo.toml`
- Create: `easy/arrays-hashing/0001-two-sum/README.md`
- Create: `easy/arrays-hashing/0001-two-sum/src/lib.rs`
- Create: `easy/arrays-hashing/0001-two-sum/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "two-sum"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Two Sum

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/two-sum/

## Problem

Given an array of integers `nums` and an integer `target`, return the indices of
the two numbers that add up to `target`. Each input has exactly one solution, and
you may not use the same element twice. The answer may be returned in any order.

## Examples

\```
Input:  nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: nums[0] + nums[1] == 9
\```
\```
Input:  nums = [3,2,4], target = 6
Output: [1,2]
\```

## Constraints

- 2 <= nums.len() <= 10^4
- -10^9 <= nums[i] <= 10^9
- Exactly one valid answer exists.

## Function signature

\```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}
\```

## Rust std hints

- `HashMap` to remember `value -> index` as you scan once.
- `HashMap::get`, `HashMap::insert`, `Iterator::enumerate`.

## Notes / approaches

One pass: for each `n`, check if `target - n` was already seen.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use two_sum::two_sum;

// Two Sum may return the pair in any order; normalize before comparing.
fn check(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
    let mut got = two_sum(nums, target);
    got.sort();
    let mut exp = expected;
    exp.sort();
    assert_eq!(got, exp);
}

#[test]
fn example_1() {
    check(vec![2, 7, 11, 15], 9, vec![0, 1]);
}

#[test]
fn example_2() {
    check(vec![3, 2, 4], 6, vec![1, 2]);
}

#[test]
fn duplicates() {
    check(vec![3, 3], 6, vec![0, 1]);
}

#[test]
fn negatives() {
    check(vec![-3, 4, 3, 90], 0, vec![0, 2]);
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j, i as i32];
        }
        seen.insert(n, i as i32);
    }
    vec![]
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p two-sum`
Expected: PASS (4 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p two-sum`
Expected: FAIL — tests panic with `not yet implemented` (`todo!`).

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml easy/arrays-hashing/0001-two-sum
git commit -m "Add Two Sum (1): stub + tests"
```

---

## Task 3: Contains Duplicate (easy / arrays-hashing / 0217-contains-duplicate)

**Files:**
- Create: `easy/arrays-hashing/0217-contains-duplicate/Cargo.toml`
- Create: `easy/arrays-hashing/0217-contains-duplicate/README.md`
- Create: `easy/arrays-hashing/0217-contains-duplicate/src/lib.rs`
- Create: `easy/arrays-hashing/0217-contains-duplicate/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "contains-duplicate"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Contains Duplicate

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/contains-duplicate/

## Problem

Given an integer array `nums`, return `true` if any value appears at least twice,
and `false` if every element is distinct.

## Examples

\```
Input:  nums = [1,2,3,1]
Output: true
\```
\```
Input:  nums = [1,2,3,4]
Output: false
\```

## Constraints

- 1 <= nums.len() <= 10^5
- -10^9 <= nums[i] <= 10^9

## Function signature

\```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}
\```

## Rust std hints

- `HashSet::insert` returns `false` when the value was already present.

## Notes / approaches

Insert each value into a `HashSet`; if an insert fails, you found a duplicate.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use contains_duplicate::contains_duplicate;

#[test]
fn has_duplicate() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
}

#[test]
fn all_distinct() {
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn many_duplicates() {
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}

#[test]
fn single_element() {
    assert!(!contains_duplicate(vec![7]));
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for n in nums {
        if !seen.insert(n) {
            return true;
        }
    }
    false
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p contains-duplicate`
Expected: PASS (4 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p contains-duplicate`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/arrays-hashing/0217-contains-duplicate
git commit -m "Add Contains Duplicate (217): stub + tests"
```

---

## Task 4: Valid Anagram (easy / arrays-hashing / 0242-valid-anagram)

**Files:**
- Create: `easy/arrays-hashing/0242-valid-anagram/Cargo.toml`
- Create: `easy/arrays-hashing/0242-valid-anagram/README.md`
- Create: `easy/arrays-hashing/0242-valid-anagram/src/lib.rs`
- Create: `easy/arrays-hashing/0242-valid-anagram/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "valid-anagram"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Valid Anagram

- **Difficulty:** Easy
- **Topic:** arrays-hashing
- **LeetCode:** https://leetcode.com/problems/valid-anagram/

## Problem

Given two strings `s` and `t`, return `true` if `t` is an anagram of `s` (the
same characters with the same counts, just reordered).

## Examples

\```
Input:  s = "anagram", t = "nagaram"
Output: true
\```
\```
Input:  s = "rat", t = "car"
Output: false
\```

## Constraints

- 1 <= s.len(), t.len() <= 5 * 10^4
- `s` and `t` consist of lowercase English letters.

## Function signature

\```rust
pub fn is_anagram(s: String, t: String) -> bool {
    todo!()
}
\```

## Rust std hints

- `HashMap<char, i32>` with `entry(c).or_insert(0)` to count.
- `String::chars`, `HashMap::values`.

## Notes / approaches

Count chars in `s` (increment) and `t` (decrement); they match iff all counts
end at zero. Differing lengths can never be anagrams.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use valid_anagram::is_anagram;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn is_an_anagram() {
    assert!(is_anagram(s("anagram"), s("nagaram")));
}

#[test]
fn not_an_anagram() {
    assert!(!is_anagram(s("rat"), s("car")));
}

#[test]
fn different_lengths() {
    assert!(!is_anagram(s("a"), s("ab")));
}

#[test]
fn same_letters_different_counts() {
    assert!(!is_anagram(s("aacc"), s("ccac")));
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        let e = counts.entry(c).or_insert(0);
        *e -= 1;
        if *e < 0 {
            return false;
        }
    }
    counts.values().all(|&v| v == 0)
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p valid-anagram`
Expected: PASS (4 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn is_anagram(s: String, t: String) -> bool {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p valid-anagram`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/arrays-hashing/0242-valid-anagram
git commit -m "Add Valid Anagram (242): stub + tests"
```

---

## Task 5: Longest Common Prefix (easy / strings / 0014-longest-common-prefix)

**Files:**
- Create: `easy/strings/0014-longest-common-prefix/Cargo.toml`
- Create: `easy/strings/0014-longest-common-prefix/README.md`
- Create: `easy/strings/0014-longest-common-prefix/src/lib.rs`
- Create: `easy/strings/0014-longest-common-prefix/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "longest-common-prefix"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Longest Common Prefix

- **Difficulty:** Easy
- **Topic:** strings
- **LeetCode:** https://leetcode.com/problems/longest-common-prefix/

## Problem

Write a function to find the longest common prefix string amongst an array of
strings. If there is no common prefix, return the empty string `""`.

## Examples

\```
Input:  strs = ["flower","flow","flight"]
Output: "fl"
\```
\```
Input:  strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
\```

## Constraints

- 1 <= strs.len() <= 200
- 0 <= strs[i].len() <= 200
- `strs[i]` consists of lowercase English letters.

## Function signature

\```rust
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    todo!()
}
\```

## Rust std hints

- `str::starts_with`, `String::pop`, `String::is_empty`.
- Slicing a `Vec` with `&strs[1..]` to iterate the rest.

## Notes / approaches

Start with the first string as the candidate prefix; for each other string,
shrink the prefix from the end until it is a prefix of that string.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use longest_common_prefix::longest_common_prefix;

fn lcp(v: &[&str]) -> String {
    longest_common_prefix(v.iter().map(|s| s.to_string()).collect())
}

#[test]
fn common_prefix() {
    assert_eq!(lcp(&["flower", "flow", "flight"]), "fl");
}

#[test]
fn no_common_prefix() {
    assert_eq!(lcp(&["dog", "racecar", "car"]), "");
}

#[test]
fn single_string() {
    assert_eq!(lcp(&["alone"]), "alone");
}

#[test]
fn all_identical() {
    assert_eq!(lcp(&["abc", "abc", "abc"]), "abc");
}

#[test]
fn empty_member_forces_empty() {
    assert_eq!(lcp(&["", "b"]), "");
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p longest-common-prefix`
Expected: PASS (5 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p longest-common-prefix`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/strings/0014-longest-common-prefix
git commit -m "Add Longest Common Prefix (14): stub + tests"
```

---

## Task 6: Reverse String (easy / strings / 0344-reverse-string)

**Files:**
- Create: `easy/strings/0344-reverse-string/Cargo.toml`
- Create: `easy/strings/0344-reverse-string/README.md`
- Create: `easy/strings/0344-reverse-string/src/lib.rs`
- Create: `easy/strings/0344-reverse-string/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "reverse-string"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Reverse String

- **Difficulty:** Easy
- **Topic:** strings
- **LeetCode:** https://leetcode.com/problems/reverse-string/

## Problem

Write a function that reverses a string. The input is given as a `Vec<char>` and
you must reverse it **in place** with O(1) extra memory.

## Examples

\```
Input:  s = ['h','e','l','l','o']
Output: ['o','l','l','e','h']
\```
\```
Input:  s = ['H','a','n','n','a','h']
Output: ['h','a','n','n','a','H']
\```

## Constraints

- 1 <= s.len() <= 10^5
- `s[i]` is a printable ASCII character.

## Function signature

\```rust
pub fn reverse_string(s: &mut Vec<char>) {
    todo!()
}
\```

## Rust std hints

- `Vec::swap(i, j)` swaps two elements without cloning.
- `Vec::len`, `usize::saturating_sub` to avoid underflow on an empty vec.

## Notes / approaches

Two pointers from both ends, swapping inward until they meet.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use reverse_string::reverse_string;

#[test]
fn reverses_hello() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}

#[test]
fn reverses_hannah() {
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}

#[test]
fn single_char() {
    let mut s = vec!['a'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['a']);
}

#[test]
fn empty() {
    let mut s: Vec<char> = vec![];
    reverse_string(&mut s);
    assert_eq!(s, Vec::<char>::new());
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

pub fn reverse_string(s: &mut Vec<char>) {
    let mut i = 0usize;
    let mut j = s.len().saturating_sub(1);
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p reverse-string`
Expected: PASS (4 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn reverse_string(s: &mut Vec<char>) {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p reverse-string`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/strings/0344-reverse-string
git commit -m "Add Reverse String (344): stub + tests"
```

---

## Task 7: Valid Palindrome (easy / two-pointers / 0125-valid-palindrome)

**Files:**
- Create: `easy/two-pointers/0125-valid-palindrome/Cargo.toml`
- Create: `easy/two-pointers/0125-valid-palindrome/README.md`
- Create: `easy/two-pointers/0125-valid-palindrome/src/lib.rs`
- Create: `easy/two-pointers/0125-valid-palindrome/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "valid-palindrome"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Valid Palindrome

- **Difficulty:** Easy
- **Topic:** two-pointers
- **LeetCode:** https://leetcode.com/problems/valid-palindrome/

## Problem

A phrase is a palindrome if, after lowercasing and removing all non-alphanumeric
characters, it reads the same forward and backward. Given a string `s`, return
`true` if it is a palindrome.

## Examples

\```
Input:  s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" reads the same both ways.
\```
\```
Input:  s = "race a car"
Output: false
\```
\```
Input:  s = " "
Output: true
Explanation: After removing non-alphanumerics it is empty, which is a palindrome.
\```

## Constraints

- 1 <= s.len() <= 2 * 10^5
- `s` consists of printable ASCII characters.

## Function signature

\```rust
pub fn is_palindrome(s: String) -> bool {
    todo!()
}
\```

## Rust std hints

- `char::is_alphanumeric`, `char::to_ascii_lowercase`.
- `Iterator::filter` + `Iterator::map` + `collect::<Vec<char>>()`.

## Notes / approaches

Normalize to a `Vec<char>` of lowercase alphanumerics, then two-pointer compare
from both ends.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use valid_palindrome::is_palindrome;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn classic_palindrome() {
    assert!(is_palindrome(s("A man, a plan, a canal: Panama")));
}

#[test]
fn not_a_palindrome() {
    assert!(!is_palindrome(s("race a car")));
}

#[test]
fn only_punctuation_is_empty() {
    assert!(is_palindrome(s(" ")));
}

#[test]
fn alphanumeric_mismatch() {
    // filters to "0p" -> '0' != 'p'
    assert!(!is_palindrome(s("0P")));
}

#[test]
fn empty_string() {
    assert!(is_palindrome(s("")));
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if chars.is_empty() {
        return true;
    }
    let mut i = 0usize;
    let mut j = chars.len() - 1;
    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p valid-palindrome`
Expected: PASS (5 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn is_palindrome(s: String) -> bool {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p valid-palindrome`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/two-pointers/0125-valid-palindrome
git commit -m "Add Valid Palindrome (125): stub + tests"
```

---

## Task 8: Valid Parentheses (easy / stack / 0020-valid-parentheses)

**Files:**
- Create: `easy/stack/0020-valid-parentheses/Cargo.toml`
- Create: `easy/stack/0020-valid-parentheses/README.md`
- Create: `easy/stack/0020-valid-parentheses/src/lib.rs`
- Create: `easy/stack/0020-valid-parentheses/tests/tests.rs`

- [ ] **Step 1: `Cargo.toml`**

```toml
[package]
name = "valid-parentheses"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
```

- [ ] **Step 2: `README.md`**

```markdown
# Valid Parentheses

- **Difficulty:** Easy
- **Topic:** stack
- **LeetCode:** https://leetcode.com/problems/valid-parentheses/

## Problem

Given a string `s` containing just the characters `()[]{}`, determine if the
input is valid. Brackets must close in the correct order and every closing
bracket must match the most recent unclosed opening bracket of the same type.

## Examples

\```
Input:  s = "()[]{}"
Output: true
\```
\```
Input:  s = "(]"
Output: false
\```
\```
Input:  s = "([{}])"
Output: true
\```

## Constraints

- 1 <= s.len() <= 10^4
- `s` consists only of the characters `()[]{}`.

## Function signature

\```rust
pub fn is_valid(s: String) -> bool {
    todo!()
}
\```

## Rust std hints

- `Vec` as a stack: `push` / `pop` (`pop` returns `Option`).
- `HashMap::from([...])` to map each closing bracket to its opener.

## Notes / approaches

Push openers; on a closer, pop and check it matches. Valid iff the stack is
empty at the end.
```

- [ ] **Step 3: `tests/tests.rs`**

```rust
use valid_parentheses::is_valid;

fn s(x: &str) -> String {
    x.to_string()
}

#[test]
fn simple_pair() {
    assert!(is_valid(s("()")));
}

#[test]
fn all_types() {
    assert!(is_valid(s("()[]{}")));
}

#[test]
fn mismatched() {
    assert!(!is_valid(s("(]")));
}

#[test]
fn nested() {
    assert!(is_valid(s("([{}])")));
}

#[test]
fn unbalanced_open() {
    assert!(!is_valid(s("(")));
}

#[test]
fn unbalanced_close() {
    assert!(!is_valid(s(")")));
}
```

- [ ] **Step 4: `src/lib.rs` (reference solution — temporary)**

```rust
//! See README.md for the problem statement.

use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let pairs: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if let Some(&open) = pairs.get(&c) {
            if stack.pop() != Some(open) {
                return false;
            }
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}
```

- [ ] **Step 5: Verify tests pass with the reference**

Run: `cargo test -p valid-parentheses`
Expected: PASS (6 passed).

- [ ] **Step 6: Replace `src/lib.rs` body with the practice stub**

```rust
//! See README.md for the problem statement.

pub fn is_valid(s: String) -> bool {
    todo!()
}
```

- [ ] **Step 7: Verify the stub fails as expected**

Run: `cargo test -p valid-parentheses`
Expected: FAIL — tests panic with `not yet implemented`.

- [ ] **Step 8: Commit**

```bash
git add easy/stack/0020-valid-parentheses
git commit -m "Add Valid Parentheses (20): stub + tests"
```

---

## Task 9: Whole-workspace sanity check + scaffold smoke test

**Files:** none created; verification only.

- [ ] **Step 1: Confirm the whole workspace builds and is consistent**

Run: `cargo build --workspace`
Expected: SUCCESS — all 7 crates compile (stubs compile fine; `todo!()` is valid code).

- [ ] **Step 2: Confirm the full test run executes (stubs panic — expected)**

Run: `cargo test --workspace`
Expected: Tests run for all 7 crates and FAIL with `not yet implemented` panics. This is the intended "everything is ready to be solved" state.

- [ ] **Step 3: Smoke-test the scaffold script (then discard the output)**

Run:
```bash
./new-problem.sh easy arrays-hashing 9999-scaffold-smoke smoke
cargo metadata --no-deps --format-version 1 >/dev/null
```
Expected: prints `Created easy/arrays-hashing/9999-scaffold-smoke ...`; `cargo metadata` succeeds, proving the new member was added correctly and the generated crate parses.

- [ ] **Step 4: Remove the smoke-test crate and its workspace entry**

Run:
```bash
rm -rf easy/arrays-hashing/9999-scaffold-smoke
git checkout -- Cargo.toml 2>/dev/null || sed -i '' '/9999-scaffold-smoke/d' Cargo.toml
```
Expected: the smoke crate and its `members` line are gone. Verify with `cargo metadata --no-deps --format-version 1 >/dev/null` (SUCCESS) and `git status` (clean except nothing related to the smoke test).

- [ ] **Step 5: Final commit (if anything is pending)**

```bash
git add -A
git commit -m "Verify workspace builds and scaffold script works" --allow-empty
```

---

## Notes on the answer key

The reference solutions in Tasks 2–8 are the answer key. They are pasted into
`src/lib.rs` only to validate each test suite (step 5 of each task), then replaced
by the `todo!()` stub (step 6) before committing. The committed working tree
contains only stubs — the exercises. If you ever want to check an answer, it lives
in this plan document, not in the code.
```

