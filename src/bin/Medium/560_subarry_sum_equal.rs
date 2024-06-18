
use std::collections::HashMap;

fn subarray_sum_equal(nums: Vec<i32>, k: i32) -> i32 {
    let mut total = 0;
    let mut count = 0;
    let mut dp: HashMap<i32, i32> = HashMap::new();

    dp.insert(0, 1);

    for num in nums {
        total += num;
        let diff = total - k;

        if let Some(diff) = dp.get(&diff) {
            count += diff;
        }
        if let Some(total) = dp.get_mut(&total) {
            *total += 1;
        } else {
            dp.insert(total, 1);
        }
    }
    count
}

fn main() {
    let nums = vec![1, 1, 1, 1, 1];
    let k = 3;
    assert_eq!(subarray_sum_equal(nums, k), 3);
}