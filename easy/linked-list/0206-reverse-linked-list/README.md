# Reverse Linked List

- **Difficulty:** Easy
- **Topic:** linked-list
- **LeetCode:** https://leetcode.com/problems/reverse-linked-list/

## Problem

Given the `head` of a singly linked list, reverse the list and return the new
head.

The crate defines the standard LeetCode `ListNode`:

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

## Examples

```
Input:  head = [1,2,3,4,5]
Output: [5,4,3,2,1]
```
```
Input:  head = [1,2]
Output: [2,1]
```
```
Input:  head = []
Output: []
```

## Constraints

- The number of nodes is in `[0, 5000]`.
- -5000 <= Node.val <= 5000

## Function signature

```rust
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}
```

## Rust std hints

- Iterative: walk the list, taking ownership of each node with `Option::take`,
  and splice it onto a growing `prev` chain.
- `head.take()` and `node.next.take()` keep the borrow checker happy because you
  move boxes rather than alias them.

## Notes / approaches

The Rust twist is ownership, not algorithm. Carry a `prev: Option<Box<ListNode>>`
accumulator; repeatedly detach the front node from `head` and push it onto
`prev`.
