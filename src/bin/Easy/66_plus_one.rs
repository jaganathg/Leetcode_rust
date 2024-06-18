fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev(){
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    let mut new_digits:Vec<i32> = Vec::new();
    new_digits.push(1);
    new_digits.append(&mut digits);
    new_digits
}


fn main() {
    let digits = vec![1, 2, 9];
    let result = plus_one(digits);
    println!("{:?}", result);
    
}