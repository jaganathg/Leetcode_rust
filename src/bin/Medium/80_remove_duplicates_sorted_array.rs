

fn remove_duplicates_sorted_array_dyn(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut write_index = 2;

    for i in 2..nums.len() {
        if nums[i] != nums[write_index - 2] {
            nums[write_index] = nums[i];
            write_index += 1;
        }
    }
    write_index as i32
}


fn main() {
    let mut nums = vec![1, 1, 1,1, 1,1, 2, 2, 3];
    let len = remove_duplicates_sorted_array_dyn(&mut nums);
    println!("{:?}", nums);
    println!("{:?}", len);
    assert_eq!(nums, vec![1, 1, 2, 2, 3, 1,2,2,3]);
    assert_eq!(len, 5);
}