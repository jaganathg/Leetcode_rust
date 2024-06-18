
fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted_heigts = heights.clone();
    sorted_heigts.sort_unstable();

    let mut count = 0;
    for i in 0..heights.len() {
        if heights[i] != sorted_heigts[i]{
            count += 1;
        }
    }
    count
}

fn _leetcode_solution(heights: Vec<i32>) -> i32 {
    let mut counter = 0;

    let mut sorted = heights.clone();
    sorted.sort_unstable();

    for (a, b) in sorted.into_iter().zip(heights.into_iter()) {
        if a != b {
            counter += 1;
        }
    }

    counter
}


fn main() {
    let heights = vec![1,1,4,2,1,3];
    let result = height_checker(heights);
    println!("{}", result);

    let heights = vec![5,1,2,3,4];
    let result = height_checker(heights);
    println!("{}", result);

    let heights = vec![1,2,3,4,5];
    let result = height_checker(heights);
    println!("{}", result);
}