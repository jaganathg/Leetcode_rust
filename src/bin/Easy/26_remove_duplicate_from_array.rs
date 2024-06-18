fn remove_duplicate(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0
    }
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    i as i32 + 1
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let result = remove_duplicate(&mut nums);
    println!("{:?}", result);
    println!("{:?}", nums);
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = remove_duplicate(&mut nums);
    println!("{:?}", result);
    println!("{:?}", nums);
}