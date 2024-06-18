
fn rotate_array(nums: &mut Vec<i32>, steps: i32) {
    let l = nums.len();
    let steps: usize = steps as usize;
    let steps: usize = steps % l ;
    if steps == 0 {
        return;
    }

    reverst(nums, 0, l - 1);
    reverst(nums, 0, steps - 1);
    reverst(nums, steps, l - 1);

}

fn reverst(nums: &mut [i32], start: usize, end: usize) {
    let mut start = start;
    let mut end = end;
    while start < end {
        nums.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_array(&mut nums, 3);
    println!("{:?}", nums);
}