fn length_last_word(s: String) -> i32 {
   let mut count = 0;
   let mut last = 0;
   for c in s.chars() {
         if c == ' ' {
              if count > 0 {
                last = count;
              }
              count = 0;
         } else {
              count += 1;
         }
   }
    if count > 0 {
         return count;
    } else {
         return last;
    }
}

fn main() {
    let s = "Hello World".to_string();
    let result = length_last_word(s);
    println!("{:?}", result);
    let s = "Hello World ".to_string();
    let result = length_last_word(s);
    println!("{:?}", result);
    let s = "Hello World  ".to_string();
    let result = length_last_word(s);
    println!("{:?}", result);
    let s = "Hello World  a".to_string();
    let result = length_last_word(s);
    println!("{:?}", result);
    let s = "Hello World  a ".to_string();
    let result = length_last_word(s);
    println!("{:?}", result);
    
}