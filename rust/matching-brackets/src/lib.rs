pub fn matching_char(a: char) -> Option<char> {
    match a {
        ']' => Some('['),
        '}' => Some('{'),
        ')' => Some('('),
        _ => unreachable!()
    }
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut result: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => result.push(c),
            ']' | '}' | ')' => if result.pop() != matching_char(c) {return false},
            _ => ()
        };
    };
    result.is_empty()

}
