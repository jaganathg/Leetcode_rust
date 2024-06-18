fn longest_common_perfix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
   
}

fn main() {
    let strs = vec!["flower".to_string(), "flight".to_string(), "flow".to_string()];
    println!("{}", longest_common_perfix(strs));
}