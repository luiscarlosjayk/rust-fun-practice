use maximum_depth_of_binary_tree::{max_depth, TreeNode};
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

#[test]
fn example_1() {
    let t = build(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(max_depth(t), 3);
}

#[test]
fn right_leaning() {
    let t = build(vec![Some(1), None, Some(2)]);
    assert_eq!(max_depth(t), 2);
}

#[test]
fn empty() {
    assert_eq!(max_depth(None), 0);
}

#[test]
fn single() {
    assert_eq!(max_depth(build(vec![Some(0)])), 1);
}
