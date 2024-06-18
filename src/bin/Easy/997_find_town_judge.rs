

fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {

    if trust.len() < n as usize {
        return -1;
    }

    let mut ppl = vec![0; n as usize + 1];

    for pair in trust {
        ppl[pair[0] as usize] -= 1;
        ppl[pair[1] as usize] += 1;
    }

    for i in 1..=n as usize {
        if ppl[i] == n - 1 {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let n = 6;
    let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
    let res = find_judge(n, trust);
    println!("{}", res);
}