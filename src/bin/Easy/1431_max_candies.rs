
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max().unwrap();
    candies.iter().map(|candy| candy + extra_candies >= *max_candies).collect()
}


fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let result = kids_with_candies(candies, extra_candies);
    println!("{:?}", result);

    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    let result = kids_with_candies(candies, extra_candies);
    println!("{:?}", result);

    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    let result = kids_with_candies(candies, extra_candies);
    println!("{:?}", result);
}