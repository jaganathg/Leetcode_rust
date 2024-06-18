fn search_index(nums: Vec<i32>, target: i32) -> i32 {
    let mut pos = 0;
    for num in nums.iter() {
        if *num >= target {
            return pos;
        }
        pos += 1;
    }
    nums.len() as i32
}

fn _best_search_index(nums: Vec<i32>, target: i32) -> i32 {
    
    let x =nums.binary_search(&target);

    match x{
        Ok(x)=> x as i32,
        Err(x)=> x as i32
    }
 
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let result = search_index(nums, target);
    println!("{:?}", result);
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    let result = search_index(nums, target);
    println!("{:?}", result);
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let result = search_index(nums, target);
    println!("{:?}", result);
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = search_index(nums, target);
    println!("{:?}", result);
}