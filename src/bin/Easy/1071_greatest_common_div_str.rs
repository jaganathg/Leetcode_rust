use std::cmp::min;

fn gcd_of_string(str1: String, str2: String) -> String {
    let len1 = str1.len();
    let len2 = str2.len();

    for l in (1..=min(len1, len2)).rev() {
            if len1 % l == 0 && len2 % l == 0 {
                
                let f1 = len1 / l;
                let f2 = len2 / l;
                let fel = &str1[..l];
                if fel.repeat(f1) == str1 && fel.repeat(f2) == str2 {
                    return fel.to_string();
            }
        }
    }
    "".to_string()
}

fn _leet_code(str1: String, str2: String) -> String {
    let version1 = format!("{}{}", str1, str2);
    let version2 = format!("{}{}", str2, str1);
    if version1 != version2 {
        return "".to_string();
    }
    let g = Self::gcd(str1.len() as u32, str2.len() as u32);
    str1.get(0..(g as usize)).unwrap().to_string()
}


fn main() {
    let str1 = "ABCABC".to_string();
    let str2 = "ABC".to_string();
    let result = gcd_of_string(str1, str2);
    println!("{}", result);

    let str1 = "ABABAB".to_string();
    let str2 = "ABAB".to_string();
    let result = gcd_of_string(str1, str2);
    println!("{}", result);

    let str1 = "LEET".to_string();
    let str2 = "CODE".to_string();
    let result = gcd_of_string(str1, str2);
    println!("{}", result);

    let str1 = "ABCDEF".to_string();
    let str2 = "ABC".to_string();
    let result = gcd_of_string(str1, str2);
    println!("{}", result);
}