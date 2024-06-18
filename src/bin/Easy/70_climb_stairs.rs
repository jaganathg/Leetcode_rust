pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut first = 1;
    let mut second = 2;
    let mut current = 0;

    for _ in 3..=n {
        current = first + second;
        first = second;
        second = current;
    }
    second
}

fn main() {
    let res = climb_stairs(5);
    println!("{:?}", res);
}