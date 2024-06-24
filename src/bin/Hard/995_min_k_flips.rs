

fn min_k_flip_bits(nums: Vec<i32>, k: i32) -> i32 {

    let mut nums = nums.clone();
    let mut res = 0;
    let mut curr_flip = 0;

    for i in 0..nums.len() {

        if i as i32 - k >= 0 && nums[i - k as usize] == 2 {
            curr_flip -= 1;
        }

        if (nums[i] + curr_flip) % 2 == 0 {
            if i + k as usize > nums.len() {
                return -1;
            }
            res += 1;
            curr_flip += 1;
            nums[i] = 2;
        }
    }
    res
}

fn main() {
    let nums = vec![0, 1, 0];
    let k = 1;
    let res = min_k_flip_bits(nums, k);
    println!("{}", res);

    let nums = vec![1, 1, 0];
    let k = 2;
    let res = min_k_flip_bits(nums, k);
    println!("{}", res);

    let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
    let k = 3;
    let res = min_k_flip_bits(nums, k);
    println!("{}", res);
}