
use std::collections::VecDeque;

fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut min_q: VecDeque<i32> = VecDeque::new();
    let mut max_q: VecDeque<i32> = VecDeque::new();

    let mut l: usize = 0;
    let mut res: i32 = 0;

    for r in 0..nums.len() {
        while !min_q.is_empty() && nums[r] < *min_q.back().unwrap() {
            min_q.pop_back();
        }
        while !max_q.is_empty() && nums[r] > *max_q.back().unwrap() {
            max_q.pop_back();
        }

        min_q.push_back(nums[r]);
        max_q.push_back(nums[r]);

        while *max_q.front().unwrap() - *min_q.front().unwrap() > limit {
            if nums[l] == *max_q.front().unwrap() {
                max_q.pop_front();
            }
            if nums[l] == *min_q.front().unwrap() {
                min_q.pop_front();
            }
            l += 1;
        }
        res = res.max((r - l + 1) as i32);
    }
    res
}


fn main() {
    let nums: Vec<i32> = vec![8, 2, 4, 7];
    let limit: i32 = 4;
    let res: i32 = longest_subarray(nums, limit);
    println!("{}", res);
}