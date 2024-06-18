
use std::collections::HashSet;

fn longest_palindrome(s: String) -> i32 {
    let mut count: HashSet<u8> = HashSet::new();
    let mut res = 0;

    for c in s.chars() {
        let c = c as u8;
        if count.contains(&c) {
            count.remove(&c);
            res += 2;
        }
        else {
            count.insert(c);
        }
    }
    if count.is_empty() 
        { res } 
    else
        { res + 1 }

}

fn main() {
    let s = "abccccdd".to_string();
    let res = longest_palindrome(s);
    println!("{}", res);
}