use reverse_string::reverse_string;

#[test]
fn reverses_hello() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o', 's'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['s', 'o', 'l', 'l', 'e', 'h']);
}

#[test]
fn reverses_hannah() {
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}

#[test]
fn single_char() {
    let mut s = vec!['a'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['a']);
}

#[test]
fn empty() {
    let mut s: Vec<char> = vec![];
    reverse_string(&mut s);
    assert_eq!(s, Vec::<char>::new());
}
