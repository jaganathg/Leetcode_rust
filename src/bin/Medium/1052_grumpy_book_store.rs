
fn max_satisfaction(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut left: usize = 0;
    let (mut satisfied, mut windows, mut max_val) = (0, 0, 0);

    for right in 0..customers.len() {
        if grumpy[right] == 1 {
            windows += customers[right];
        } else {
            satisfied += customers[right];
        }

        if right as i32 - left as i32 + 1 > minutes {
            if grumpy[left] == 1 {
                windows -= customers[left];
            }
            left += 1;
        }
        max_val = max_val.max(windows);
    }
    satisfied + max_val
}


fn main() {
    let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let minutes = 3;
    assert_eq!(max_satisfaction(customers, grumpy, minutes), 16);
    println!("max_satisfaction passed");
}