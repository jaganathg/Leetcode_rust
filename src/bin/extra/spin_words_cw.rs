
fn spin_words(words: &str) -> String {
    words.split_whitespace()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let words = "Hey fellow warriors";
    let result = spin_words(words);
    println!("{:?}", result);
}