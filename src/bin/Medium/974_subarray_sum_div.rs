use std::collections::HashMap;

fn subarray_sum_div(nums: Vec<i32>, k: i32) -> i32 {
    let mut total = 0;
    let mut count = 0;
    let mut dp: HashMap<i32, i32> = HashMap::new();

    dp.insert(0, 1);

    for num in nums {
        total += num;
        let mut rem = total % k;

        if rem < 0 {
            rem += k;
        }

        if let Some(rem) = dp.get(&rem) {
            count += rem;
        }
        *dp.entry(rem).or_insert(0) += 1;
    }
    count
}

fn leetcode_solution(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut freq = vec![0; k as usize];
    let mut count = 0;
    
    // Initialize frequency array to handle the case for subarray starting from index 0
    freq[0] = 1;
    
    for num in nums {
        sum += num;
        // Normalize the modulus to be always positive
        let mod_k = ((sum % k) + k) % k;
        count += freq[mod_k as usize];
        freq[mod_k as usize] += 1;
    }

    count
}


fn main() {
    let nums = vec![-1, 2, 9];
    let k = 2;
    println!("{:?}", subarray_sum_div(nums, k));
    // assert_eq!(subarray_sum_div(nums, k), 2);
}