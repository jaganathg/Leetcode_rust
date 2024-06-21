

fn jump_game(nums: Vec<i32>) -> bool {
    let mut des = nums.len() - 1;

    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize >= des {
            des = i;
        }
    }
    des == 0
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let result = jump_game(nums);
    println!("{}", result);

    let nums = vec![3, 2, 1, 0, 4];
    let result = jump_game(nums);
    println!("{}", result);
}