

fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut jobs: Vec<(i32, i32)> = difficulty.into_iter()
        .zip(profit.into_iter()).collect();

    jobs.sort_unstable();
    let mut result = 0;
    let mut idx = 0;
    let mut maxp = 0;

    let mut worker = worker;
    worker.sort_unstable();

    for work in worker {
        while idx < jobs.len() && work >= jobs[idx].0 {
            maxp = maxp.max(jobs[idx].1);
            idx += 1;
        }
        result += maxp;
    }
    result
}

fn main() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    let result = max_profit_assignment(difficulty, profit, worker);
    println!("{:?}", result);
}