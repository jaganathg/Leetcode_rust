fn roman_to_number(s: String) -> i32 {
    let mut result = 0;
    let mut prev = 0;
    for c in s.chars() {
        let curr = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        result += if curr > prev { curr - 2 * prev } else { curr };
        prev = curr;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_number() {
        assert_eq!(roman_to_number("III".to_string()), 3);
        assert_eq!(roman_to_number("IV".to_string()), 4);
        assert_eq!(roman_to_number("IX".to_string()), 9);
        assert_eq!(roman_to_number("LVIII".to_string()), 58);
        assert_eq!(roman_to_number("MCMXCIV".to_string()), 1994);
    }
}

fn main() {
    println!("{}", roman_to_number("III".to_string()));
}