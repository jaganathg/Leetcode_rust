
fn patch_arrays(nums: Vec<i32>, n: i32) -> i32 {
    let mut patche_count = 0;
    let mut miss = 1;
    let mut idx = 0;

    while miss <= n {
        if idx < nums.len() && nums[idx] <= miss {
            miss += nums[idx];
            idx += 1;
        } else {
            miss += miss;
            patche_count += 1;
        }
    }
    patche_count
}

pub fn leetcode_sol(nums: Vec<i32>, n: i32) -> i32 {
    let     n     = n as i64;
    let mut count = 0;
    let mut bound = 1_i64;
    let mut nums  = nums.into_iter().map(|num| num as i64)
                        .take_while(|&num| num <= n);
    
    while bound <= n {
        if let Some(num) = nums.next() {
            while num > bound {
                count += 1;
                bound += bound;
            }
            bound += num;
        } else { 
            count += 1;
            bound += bound;
        }
    }
    count
}

fn main() {
    let nums = vec![1, 3];
    let n = 6;
    let result = patch_arrays(nums, n);
    println!("Result: {}", result);

    let nums = vec![1, 5, 10];
    let n = 20;
    let result = patch_arrays(nums, n);
    println!("Result: {}", result);
}