# Maximum Depth of Binary Tree

- **Difficulty:** Easy
- **Topic:** trees
- **LeetCode:** https://leetcode.com/problems/maximum-depth-of-binary-tree/

## Problem

Given the `root` of a binary tree, return its maximum depth: the number of nodes
along the longest path from the root down to the farthest leaf.

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
Input:  root = [3,9,20,null,null,15,7]
Output: 3
```
```
Input:  root = [1,null,2]
Output: 2
```
```
Input:  root = []
Output: 0
```

## Constraints

- The number of nodes is in `[0, 10^4]`.
- -100 <= Node.val <= 100

## Function signature

```rust
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
}
```

## Rust std hints

- Recursion is natural: `0` for an empty subtree, otherwise
  `1 + max(left, right)`.
- Borrow the node with `node.borrow()` and `Rc::clone` each child into the
  recursive call.

## Notes / approaches

A direct depth-first recursion. An iterative BFS counting levels also works if
you'd rather avoid recursion.
