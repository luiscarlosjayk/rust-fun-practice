# Invert Binary Tree

- **Difficulty:** Easy
- **Topic:** trees
- **LeetCode:** https://leetcode.com/problems/invert-binary-tree/

## Problem

Given the `root` of a binary tree, invert it (mirror left/right at every node) and
return its root.

The crate defines the standard LeetCode `TreeNode`:

```rust
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
```

Trees in the examples are written in LeetCode level-order, where `null` marks a
missing child.

## Examples

```
Input:  root = [4,2,7,1,3,6,9]
Output: [4,7,2,9,6,3,1]
```
```
Input:  root = [2,1,3]
Output: [2,3,1]
```
```
Input:  root = []
Output: []
```

## Constraints

- The number of nodes is in `[0, 100]`.
- -100 <= Node.val <= 100

## Function signature

```rust
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
```

## Rust std hints

- Shared ownership means `Rc<RefCell<TreeNode>>`: borrow a node with
  `node.borrow_mut()` and `std::mem::swap` its `left` and `right`.
- Recurse into both children (or push them on a stack/queue for an iterative
  walk).

## Notes / approaches

Swap children at the current node, then invert each subtree. `Rc::clone` to hand
a child to the recursive call without moving it out of its parent.
