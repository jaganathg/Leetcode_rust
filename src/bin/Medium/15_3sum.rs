fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut res: Vec<Vec<i32>> = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            match sum {
                _ if sum > 0 => {r -= 1} ,
                _ if sum < 0 => {l += 1} ,
                _ => { 
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
        }
    }
    res
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res = three_sum(nums);
    println!("{:?}", res);
}