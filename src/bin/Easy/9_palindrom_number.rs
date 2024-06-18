fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut num = x;
    let mut rev = 0;
    while num != 0 {
        let digit = num % 10;
        rev = rev * 10 + digit;
        num /= 10;
    }
    x == rev
}


fn _reverse(mut x: i32) -> i32 {
        let mut y = 0;
        while x > 0 {
            let m = x % 10;
            let d = x / 10;

            y = 10 * y + m;
            x = d;
        }
        y
    }


fn _is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        x == _reverse(x)
    }


fn main() {
    assert!(is_palindrome(121) );
    assert!(is_palindrome(-121) );
    assert!(is_palindrome(10) );
    assert!(is_palindrome(-101) );
    assert!(is_palindrome(0));
    println!("is_palindrome passed")

}
