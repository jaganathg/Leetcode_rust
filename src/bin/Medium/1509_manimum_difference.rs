

fn min_difference(mut nums: Vec<i32>) -> i32 {

    if nums.len() <= 4 {
        return 0;
    }

    nums.sort();

    let mut min = i32::MAX;

    for i in 0..4 {
        min = min.min(nums[nums.len() - 4 + i] - nums[i]);
    }
    min      
}



fn main() {
    let nums1 = vec![5, 3, 2, 4];
    let nums2 = vec![1, 5, 0, 10, 14];
    let nums3 = vec![3, 100, 20];
    
    println!("{}", min_difference(nums1)); // Output: 0
    println!("{}", min_difference(nums2)); // Output: 1
    println!("{}", min_difference(nums3)); // Output: 0
}