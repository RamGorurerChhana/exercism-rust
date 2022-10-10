pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if let Some('(') = stack.pop() {
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some('{') = stack.pop() {
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some('[') = stack.pop() {
                } else {
                    return false;
                }
            }

            _ => {}
        }
    }

    stack.is_empty()
}
