use group_anagrams::group_anagrams;

/// Groups and their contents may come back in any order. Normalize by sorting
/// each group's words and then sorting the list of groups before comparing.
fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for g in groups.iter_mut() {
        g.sort();
    }
    groups.sort();
    groups
}

fn check(input: Vec<&str>, expected: Vec<Vec<&str>>) {
    let got = group_anagrams(input.into_iter().map(String::from).collect());
    let exp: Vec<Vec<String>> = expected
        .into_iter()
        .map(|g| g.into_iter().map(String::from).collect())
        .collect();
    assert_eq!(normalize(got), normalize(exp));
}

#[test]
fn example_1() {
    check(
        vec!["eat", "tea", "tan", "ate", "nat", "bat"],
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
    );
}

#[test]
fn empty_string() {
    check(vec![""], vec![vec![""]]);
}

#[test]
fn single_char() {
    check(vec!["a"], vec![vec!["a"]]);
}

#[test]
fn no_anagrams() {
    check(vec!["abc", "def", "ghi"], vec![vec!["abc"], vec!["def"], vec!["ghi"]]);
}
