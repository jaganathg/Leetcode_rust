fn my_sqrt(x: i32) -> i32 {
    let mut left = 1;
    let mut right = x;

    if right == 0 {
        return 0;
    }
    if right == 1 {
        return 1;
    }
    while left < right {
        let mid = (left + right) / 2;
        if mid * mid == x {
            return mid;
        } else if mid * mid < x{
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1 
}

fn main() {
    let x = 8;
    let result = my_sqrt(x);
    println!("{:?}", result);
    let x = 4;
    let result = my_sqrt(x);
    println!("{:?}", result);
    let x = 0;
    let result = my_sqrt(x);
    println!("{:?}", result);
    let x = 1;
    let result = my_sqrt(x);
    println!("{:?}", result);
}