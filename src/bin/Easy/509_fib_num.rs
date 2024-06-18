
fn fib(n: i32) -> i32 {
    if n > 0 {
        if n == 1 {
            return 1;
        }
        else {
            let mut prev = 0;
            let mut curr = 1;
            // let mut temp = 0;
            for _ in 2..=n {
                curr = prev + curr;
                prev = curr - prev;
            }
            return curr;
        }
    }   
    0   
}

fn main() {
    let n = 5;
    let result = fib(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}