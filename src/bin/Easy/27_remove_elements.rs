fn remove_elements(nums: &mut Vec<i32>, val: i32) -> i32 {
    /*if nums.len() == 0 { 
        return 0;
    }*/
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    slow as i32
}

fn remove_elements_remove(nums: &mut Vec<i32>, val: i32) -> Vec<i32> {
    nums.retain(|&x| x != val);
    nums.clone()

}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let result = remove_elements_remove(&mut nums, 3);
    println!("{:?}", result);
    println!("{:?}", nums);
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let result = remove_elements_remove(&mut nums, 2);
    println!("{:?}", result);
    println!("{:?}", nums);
}