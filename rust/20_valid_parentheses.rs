impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                _ => match stack.pop() {
                    Some('(') if c == ')' => (),
                    Some('{') if c == '}' => (),
                    Some('[') if c == ']' => (),
                    _ => return false,
                },
            }
        }

        stack.is_empty()
    }
}
