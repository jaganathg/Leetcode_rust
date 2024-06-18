

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut sell = 0;

    for price in prices {
        buy = buy.min(price);
        sell = sell.max(price - buy);
    }
    sell
}


fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = max_profit(prices);
    println!("{}", result);

    let prices = vec![7, 6, 4, 3, 1];
    let result = max_profit(prices);
    println!("{}", result);
}