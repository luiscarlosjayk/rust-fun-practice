# Merge k Sorted Lists

- **Difficulty:** Hard
- **Topic:** linked-list
- **LeetCode:** https://leetcode.com/problems/merge-k-sorted-lists/

## Problem

You are given an array of `k` linked lists, each sorted in non-decreasing order.
Merge them all into one sorted linked list and return its head.

The crate defines the standard LeetCode `ListNode`:

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

## Examples

```
Input:  lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
```
```
Input:  lists = []
Output: []
```
```
Input:  lists = [[]]
Output: []
```

## Constraints

- `k == lists.len()`
- 0 <= k <= 10^4
- 0 <= lists[i].len() <= 500
- -10^4 <= lists[i][j] <= 10^4
- Each `lists[i]` is sorted in non-decreasing order.

## Function signature

```rust
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    todo!()
}
```

## Rust std hints

- A `BinaryHeap` over the current heads gives `O(N log k)`. Wrap nodes so the heap
  pops the **smallest** `val` first — use `std::cmp::Reverse` or a custom `Ord`.
- Alternatively, fold the list with a pairwise "merge two lists" routine.
- A dummy head plus a `tail` cursor keeps the splicing loop clean.

## Notes / approaches

Two natural routes: (1) push every head into a min-heap and repeatedly pop the
smallest, pushing its `next`; or (2) reduce the vector by repeatedly merging two
lists at a time. The heap version is `O(N log k)`; pairwise reduction is also
`O(N log k)` if you merge in halves.
