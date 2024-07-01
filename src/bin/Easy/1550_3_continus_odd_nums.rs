
fn three_consecutive_odds(nums: Vec<i32>) -> bool {
    let mut count = 0;

    for i in nums.iter() {
        if i % 2 == 1 {
            count += 1;
            if count == 3 {
                return true;
            }
        }
        else {
            count = 0;
        }
    }
    false
}



fn main() {
    // Example 1
    let nums = vec![2, 6, 4, 1];
    let result = three_consecutive_odds(nums);
    println!("{}", result);

    // Example 2
    let nums = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
    let result = three_consecutive_odds(nums);
    println!("{}", result);
}