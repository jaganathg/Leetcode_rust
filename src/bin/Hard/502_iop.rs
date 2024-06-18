use std::collections::BinaryHeap;



fn find_max_profit(no_proj: i32, cap: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits.into_iter()).collect();
    projects.sort_by_key(|&(c, _)| c);
    
    let mut max_heap = BinaryHeap::new();
    let mut current_capital = cap;
    let mut index = 0;
    
    for _ in 0..no_proj {
        while index < projects.len() && projects[index].0 <= current_capital {
            max_heap.push(projects[index].1);
            index += 1;
        }
        
        if let Some(profit) = max_heap.pop() {
            current_capital += profit;
        } else {
            break;
        }
    }
    current_capital
}

fn main() {
    let no_proj = 2;
    let cap = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];
    let result = find_max_profit(no_proj, cap, profits, capital);
    println!("Maximum profit: {}", result);
}