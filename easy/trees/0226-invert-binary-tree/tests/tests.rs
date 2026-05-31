use invert_binary_tree::{invert_tree, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

/// Build a tree from a LeetCode level-order list (`None` == null child).
fn build(vals: Vec<Option<i32>>) -> Link {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while let Some(node) = queue.pop_front() {
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(Rc::clone(&child));
                queue.push_back(child);
            }
            i += 1;
        }
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().right = Some(Rc::clone(&child));
                queue.push_back(child);
            }
            i += 1;
        }
    }
    Some(root)
}

/// Serialize back to level-order, trimming trailing nulls so comparisons match
/// LeetCode's canonical form.
fn serialize(root: Link) -> Vec<Option<i32>> {
    let mut out = Vec::new();
    let mut queue: VecDeque<Link> = VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        match node {
            Some(n) => {
                out.push(Some(n.borrow().val));
                queue.push_back(n.borrow().left.clone());
                queue.push_back(n.borrow().right.clone());
            }
            None => out.push(None),
        }
    }
    while matches!(out.last(), Some(None)) {
        out.pop();
    }
    out
}

fn check(input: Vec<Option<i32>>, expected: Vec<Option<i32>>) {
    assert_eq!(serialize(invert_tree(build(input))), expected);
}

#[test]
fn example_1() {
    check(
        vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)],
        vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)],
    );
}

#[test]
fn example_2() {
    check(
        vec![Some(2), Some(1), Some(3)],
        vec![Some(2), Some(3), Some(1)],
    );
}

#[test]
fn empty() {
    check(vec![], vec![]);
}

#[test]
fn single() {
    check(vec![Some(1)], vec![Some(1)]);
}
