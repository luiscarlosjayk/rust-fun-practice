//! See README.md for the problem statement.

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for s_char in s.chars() {
        match s_char {
            '(' | '{' | '[' => stack.push(s_char),
            ')'=> if stack.pop() != Some('(') {
                return false;
            }
            '}'=> if stack.pop() != Some('{') {
                return false;
            }
            ']'=> if stack.pop() != Some('[') {
                return false;
            }
            _ => return false,
        }
    }

    stack.is_empty()
}
