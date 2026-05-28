use longest_common_prefix::longest_common_prefix;

fn lcp(v: &[&str]) -> String {
    longest_common_prefix(v.iter().map(|s| s.to_string()).collect())
}

#[test]
fn common_prefix() {
    assert_eq!(lcp(&["flower", "flow", "flight"]), "fl");
}

#[test]
fn no_common_prefix() {
    assert_eq!(lcp(&["dog", "racecar", "car"]), "");
}

#[test]
fn single_string() {
    assert_eq!(lcp(&["alone"]), "alone");
}

#[test]
fn all_identical() {
    assert_eq!(lcp(&["abc", "abc", "abc"]), "abc");
}

#[test]
fn empty_member_forces_empty() {
    assert_eq!(lcp(&["", "b"]), "");
}
