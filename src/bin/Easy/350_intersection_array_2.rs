use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut c1 = HashMap::new();
    let mut c2 = HashMap::new();

    // Count occurrences in nums1
    for num in nums1 {
        *c1.entry(num).or_insert(0) += 1;
    }

    // Count occurrences in nums2
    for num in nums2 {
        *c2.entry(num).or_insert(0) += 1;
    }

    // Find intersection based on minimum counts
    for (key, count1) in &c1 {
        if let Some(&count2) = c2.get(key) {
            let min_count = count1.min(&count2);
            for _ in 0..*min_count {
                res.push(*key);
            }
        }
    }

    res
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    println!("{:?}", intersect(nums1, nums2)); // Output: [2, 2]

    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    println!("{:?}", intersect(nums1, nums2)); // Output: [4, 9]
}