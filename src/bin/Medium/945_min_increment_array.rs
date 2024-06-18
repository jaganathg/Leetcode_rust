

fn min_increment_for_unique( nums: &mut Vec<i32>) -> i32 {

    nums.sort();
    let mut count = 0;

    for idx in 1..nums.len() {
        if nums[idx] <= nums[idx - 1] {
            let increment = nums[idx - 1] - nums[idx] + 1;
            nums[idx] += increment;
            count += increment;
        }
        // nums[idx - 1] = nums[idx];
    }
    count
} 

fn main() {
    let mut nums = vec![3, 2, 1, 2, 1, 7];
    let count = min_increment_for_unique( &mut nums);
    println!("{:?}", nums);
    println!("{:?}", count);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 7]);
    assert_eq!(count, 6);
}