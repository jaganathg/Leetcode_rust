

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max_profit += prices[i] - prices[i - 1];
        }
    }
    max_profit
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 7);
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(max_profit(prices), 4);
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(max_profit(prices), 0);
    println!("max_profit passed");
}