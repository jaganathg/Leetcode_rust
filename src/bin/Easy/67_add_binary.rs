fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();
    loop {
        let a_val = a.next().unwrap_or('0').to_digit(10).unwrap();
        let b_val = b.next().unwrap_or('0').to_digit(10).unwrap();
        let sum = a_val + b_val + carry;
        if sum == 0 {
            result.push('0');
            carry = 0;
        } else if sum == 1 {
            result.push('1');
            carry = 0;
        } else if sum == 2 {
            result.push('0');
            carry = 1;
        } else if sum == 3 {
            result.push('1');
            carry = 1;
        }
        if a.clone().next().is_none() && b.clone().next().is_none() {
            break;
        }
    }
    if carry == 1 {
        result.push('1');
    }
    result.chars().rev().collect()
    
}


fn main() {
    let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    let result = add_binary(a, b);
    println!("{}", result);
}