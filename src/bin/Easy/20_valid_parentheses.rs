
fn valid_parentheses(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

fn main() {
    let s = "()[{}]".to_string();
    println!("{}", valid_parentheses(s));
    
}