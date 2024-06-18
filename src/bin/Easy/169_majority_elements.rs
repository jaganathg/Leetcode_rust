

fn majority_elements(mut nums: Vec<i32>) -> i32 {
    let mid = nums.len() / 2;
    nums.sort();

    nums[mid]
}


fn main() {
    let nums = vec![3, 2, 3];
    let result = majority_elements(nums);
    println!("{}", result);

    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let result = majority_elements(nums);
    println!("{}", result);
}