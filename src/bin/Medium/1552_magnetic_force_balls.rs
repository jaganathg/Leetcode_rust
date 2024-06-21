

fn check(position: &Vec<i32>, mid: i32, m:i32) -> i32{
    let mut count: i32 = 1;
    let mut last_position = position[0];

    for i in 1..position.len() {
        if position[i] - last_position >= mid {
            count += 1;
            last_position = position[i];
        }
    }
    count
}

fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort();
    let (mut left, mut right) = (1, position[position.len() - 1] - position[0]);

    while left <= right {
        let mid = left + (right - left) / 2;
        if check(&position, mid, m) >= m {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    right
}

fn main() {
    let position = vec![1, 2, 3, 4, 7];
    let m = 3;
    let result = max_distance(position, m);
    println!("{}", result);
}