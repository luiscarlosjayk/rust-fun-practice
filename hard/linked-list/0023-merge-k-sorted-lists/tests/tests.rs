use merge_k_sorted_lists::{merge_k_lists, ListNode};

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

fn check(lists: Vec<Vec<i32>>, expected: Vec<i32>) {
    let input = lists.into_iter().map(from_vec).collect();
    assert_eq!(to_vec(merge_k_lists(input)), expected);
}

#[test]
fn example_1() {
    check(
        vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
        vec![1, 1, 2, 3, 4, 4, 5, 6],
    );
}

#[test]
fn no_lists() {
    check(vec![], vec![]);
}

#[test]
fn single_empty_list() {
    check(vec![vec![]], vec![]);
}

#[test]
fn some_empty_some_not() {
    check(vec![vec![], vec![1], vec![]], vec![1]);
}

#[test]
fn single_list() {
    check(vec![vec![1, 2, 3]], vec![1, 2, 3]);
}
