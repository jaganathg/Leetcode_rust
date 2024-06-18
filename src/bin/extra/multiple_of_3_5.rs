
fn multiple_of_3_5( number: i32) -> i32 {
    let mut sum = 0;
    for i in 0..number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    let number = 10;
    let result = multiple_of_3_5(number);
    println!("{:?}", result);
    let number = 16;
    let result = multiple_of_3_5(number);
    println!("{:?}", result);
    let number = 20;
    let result = multiple_of_3_5(number);
    println!("{:?}", result);
}