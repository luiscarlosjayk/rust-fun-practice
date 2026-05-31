# Merge Two Sorted Lists

- **Difficulty:** Easy
- **Topic:** linked-list
- **LeetCode:** https://leetcode.com/problems/merge-two-sorted-lists/

## Problem

You are given the heads of two sorted linked lists `list1` and `list2`. Splice
them into one sorted list by reusing the nodes, and return the head of the merged
list.

The crate defines the standard LeetCode `ListNode`:

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

## Examples

```
Input:  list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
```
```
Input:  list1 = [], list2 = []
Output: []
```
```
Input:  list1 = [], list2 = [0]
Output: [0]
```

## Constraints

- The number of nodes in each list is in `[0, 50]`.
- -100 <= Node.val <= 100
- Both lists are sorted in non-decreasing order.

## Function signature

```rust
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}
```

## Rust std hints

- A dummy head node plus a `tail` cursor (`&mut Option<Box<ListNode>>`) avoids
  special-casing the first element.
- Move boxes with `Option::take`; compare `.val` to pick the smaller front.

## Notes / approaches

Repeatedly attach the smaller of the two current heads to the tail. A recursive
solution is also clean here if you prefer it over the dummy-head loop.
