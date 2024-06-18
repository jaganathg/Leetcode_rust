
use std::collections::HashMap;

fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    // create empty hashmap
    let mut map = HashMap::new();

    // enumerate over the nums vector
    for (i, &num) in nums.iter().enumerate() {
        // get the difference between the target and the current number
        let diff = target - num;
        // if the difference is in the hashmap, return the index of the difference and the current index
        if map.contains_key(&diff){
            return vec![*map.get(&diff).unwrap() as i32, i as i32]
        }
        // insert the current number and its index into the hashmap
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    let nums = vec![2, 3, 6, 7];
    let target = 9;
    let result = twosum(nums, target);
    println!("{:?}", result);
}
