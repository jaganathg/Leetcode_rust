
fn sort_colors(nums: &mut Vec<i32>) {
    let mut left = 0;
    let mut cnt = 0;
    let mut right = nums.len() - 1;

    while cnt <= right {
        if nums[cnt] == 0 {
            nums.swap(cnt, left );
            left += 1;
            cnt += 1;
        } else if nums[cnt] == 2 {
            nums.swap(cnt, right);
            right -= 1;
        } else {
            cnt += 1;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 2, 1, 0, 2];
    sort_colors(&mut nums);
    println!("{:?}", nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
}