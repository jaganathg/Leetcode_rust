
fn merge_str_alternate(word1: String, word2: String) -> String {
    let mut result = String::new();
    let mut len = 0;
    
    for i in 0..std::cmp::min(word1.len(), word2.len()) {
        result.push(word1.chars().nth(i).unwrap());
        result.push(word2.chars().nth(i).unwrap());
        len = i;
    }
    if word1.len() > 0 {
        result.push_str(&word1[len+1..]);
    }
    if word2.len() > 0 {
        result.push_str(&word2[len+1..]);
    }
    result

}

pub fn leet_code(word1: String, word2: String) -> String {
    let mut result = vec![];
    let (mut word1_chars, mut word2_chars) = (word1.chars().peekable(), word2.chars().peekable());

    while word1_chars.peek().is_some() || word2_chars.peek().is_some() {
        match word1_chars.next() {
            Some(c) => result.push(c),
            None => {}
        };
        match word2_chars.next() {
            Some(c) => result.push(c),
            None => {}
        };
    }
    result.into_iter().collect()
}

fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    let result = merge_str_alternate(word1, word2);
    println!("{}", result);

    let word1 = "ab".to_string();
    let word2 = "pqrs".to_string();
    let result = merge_str_alternate(word1, word2);
    println!("{}", result);

    let word1 = "abcd".to_string();
    let word2 = "pq".to_string();
    let result = merge_str_alternate(word1, word2);
    println!("{}", result);
}