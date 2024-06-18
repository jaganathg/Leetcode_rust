

fn judge_square_sum(c: i32) -> bool {
    let mut left = 0i64;
    let mut right = (c as f64).sqrt() as i64;

    while left <= right {
        let sum = left * left + right * right;
        if sum == c as i64 {
            return true;
        } else if sum < c as i64 {
            left += 1;
        } else {
            right -= 1;
        }
    }
    false
}

fn main() {
    let c = 2147483600;
    let result = judge_square_sum(c);
    println!("{:?}", result);
}