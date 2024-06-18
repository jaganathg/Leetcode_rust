
fn rob(nums: Vec<i32>) -> i32 {

    let length = nums.len();

    if length == 1 {
        return nums[0];
    }
    if length == 2 {
        return nums[0].max(nums[1]);
    }

    let mut prev = nums[0];
    let mut curr = nums[0].max(nums[1]);

    for i in 2..length {
        let temp = curr;
        curr = curr.max(prev + nums[i]);
        prev = temp;
    }
    curr
}

fn main() {
    // Example usage:
    let nums = vec![2, 7, 9, 3, 1];
    let result = rob(nums);
    println!("{}", result);
}