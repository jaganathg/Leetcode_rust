use std::collections::HashMap;
use std::cmp::min;

fn common_char(words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut ref_map: HashMap<char, i32> = HashMap::new();

    for (i, word) in words.iter().enumerate() {
        let mut local_map: HashMap<char, i32> = HashMap::new();
        for c in word.chars(){
            *local_map.entry(c).or_insert(0) += 1;
        }

        if i == 0 {
            ref_map = local_map;
        } else {
            for (key, val ) in ref_map.iter_mut() {
                if let Some(&count) = local_map.get(key) {
                    *val = min(*val, count);
                } else {
                    *val = 0;
                }
            }
        }
    }
    for (key, val) in ref_map.iter_mut() {
        for _ in 0..*val {
            res.push(key.to_string());
        }
    }
    res.sort();
    res
}

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut common_chars: [usize; 26] = [0; 26];
    
    // Initialize common_chars with counts from the first word
    for c in words[0].chars() {
        common_chars[(c as u8 - b'a') as usize] += 1;
    }
    
    // Iterate through remaining words
    for word in words.iter().skip(1) {
        let mut word_chars: [usize; 26] = [0; 26];
        for c in word.chars() {
            word_chars[(c as u8 - b'a') as usize] += 1;
        }
        // Update common_chars by taking the minimum of existing count and count in current word
        for i in 0..26 {
            common_chars[i] = common_chars[i].min(word_chars[i]);
        }
    }
    
    // Construct the result array
    let mut result = Vec::new();
    for i in 0..26 {
        for _ in 0..common_chars[i] {
            result.push(((b'a' + i as u8) as char).to_string());
        }
    }
    
    result
}

fn main() {
    let words = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
    let res = common_char(words);
    println!("{:?}", res);
}