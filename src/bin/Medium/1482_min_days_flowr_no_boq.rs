
fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if bloom_day.len() < m as usize * k as usize {
        return -1;
    }
    let mut left = 1;
    let mut right = *bloom_day.iter().max().unwrap();

    while left < right {
        let mid = (left + right) / 2;
        if check_bloom(&bloom_day, m, k, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn check_bloom(bloom_day: &Vec<i32>, m: i32, k: i32, mid: i32) -> bool {
    let (mut boq, mut flr) = (0, 0);
    for &day in bloom_day {
        if day <= mid {
            flr += 1;
            if flr == k {
                boq += 1;
                flr = 0;
            }
        } else {
            flr = 0;
        }
    }
    boq >= m
}


fn main() {
    let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
    let m = 2;
    let k = 3;
    let res = min_days(bloom_day, m, k);
    println!("{}", res);
}