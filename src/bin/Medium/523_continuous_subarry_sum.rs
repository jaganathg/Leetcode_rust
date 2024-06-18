use std::collections::HashMap;

fn check_subarray_sun(nums: Vec<i32>, k: i32) -> bool {
    let mut total = 0;
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(0, -1);

    for (i, n) in nums.iter().enumerate() {
        total += n;
        let r = if k == 0 { total } else { total % k};
        if let Some(&j) = dp.get(&r) {
            if i as i32 - j > 1 {
                return true;
            }
        } else {
            dp.insert(r, i as i32);
        }
    }
    false
}

fn main() {
    let nums = vec![23, 2, 4, 6, 7];
    let k = 6;
    assert_eq!(check_subarray_sun(nums, k), true);

    let nums = vec![23, 2, 6, 4, 7];
    let k = 6;
    assert_eq!(check_subarray_sun(nums, k), true);

    let nums = vec![23, 2, 6, 4, 7];
    let k = 13;
    assert_eq!(check_subarray_sun(nums, k), false);
}