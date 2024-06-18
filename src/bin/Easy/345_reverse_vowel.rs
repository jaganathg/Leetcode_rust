
fn reverse_vowel(s: String) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = s.len() - 1;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    while left < right {
        if !vowels.contains(&s[left]){
            left += 1;
        }
        if !vowels.contains(&s[right]){
            right -= 1;
        }
        if vowels.contains(&s[left]) && vowels.contains(&s[right]) {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    s.iter().collect()
}

fn main() {
    let s = "hello".to_string();
    let result = reverse_vowel(s);
    println!("{}", result);

    let s = "leetcode".to_string();
    let result = reverse_vowel(s);
    println!("{}", result);
}

