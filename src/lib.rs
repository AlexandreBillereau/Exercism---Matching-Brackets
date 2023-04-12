pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let open_brackets = vec!['[', '{', '('];
    let close_brackets = vec![']', '}', ')'];
    
    for c in string.chars() {
        if open_brackets.contains(&c) {
            stack.push(c);
        } else if close_brackets.contains(&c) {
            if let Some(last) = stack.pop() {
                let index = open_brackets.iter().position(|&x| x == last).unwrap();
                if close_brackets[index] != c {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    
    stack.is_empty()
}
