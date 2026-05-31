use reverse_linked_list::{reverse_list, ListNode};

/// Build a list from a Vec so the tests read naturally.
fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &x in v.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

/// Flatten a list back to a Vec for comparison.
fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}

#[test]
fn example_1() {
    assert_eq!(to_vec(reverse_list(from_vec(vec![1, 2, 3, 4, 5]))), vec![5, 4, 3, 2, 1]);
}

#[test]
fn two_nodes() {
    assert_eq!(to_vec(reverse_list(from_vec(vec![1, 2]))), vec![2, 1]);
}

#[test]
fn empty() {
    assert_eq!(to_vec(reverse_list(None)), Vec::<i32>::new());
}

#[test]
fn single() {
    assert_eq!(to_vec(reverse_list(from_vec(vec![7]))), vec![7]);
}
