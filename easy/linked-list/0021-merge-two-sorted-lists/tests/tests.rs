use merge_two_sorted_lists::{merge_two_lists, ListNode};

fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &x in v.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}

fn check(a: Vec<i32>, b: Vec<i32>, expected: Vec<i32>) {
    let got = merge_two_lists(from_vec(a), from_vec(b));
    assert_eq!(to_vec(got), expected);
}

#[test]
fn example_1() {
    check(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn both_empty() {
    check(vec![], vec![], vec![]);
}

#[test]
fn one_empty() {
    check(vec![], vec![0], vec![0]);
}

#[test]
fn disjoint_ranges() {
    check(vec![1, 2, 3], vec![4, 5, 6], vec![1, 2, 3, 4, 5, 6]);
}
