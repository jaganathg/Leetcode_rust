
fn merge_sorted_array(num1: &mut Vec<i32>, m: i32, num2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut last = m + n - 1;

    while i >= 0 && j >= 0 {
        if num1[i as usize] > num2[j as usize] {
            num1[last as usize] = num1[i as usize];
            i -= 1;
        } else {
            num1[last as usize] = num2[j as usize];
            j -= 1;
        }
        last -= 1;
    }

    while j >= 0 {
        num1[last as usize] = num2[j as usize];
        j -= 1;
        last -= 1;
    }

    num1.clone()
}

fn main() {
    let mut num1 = vec![1, 2, 3, 0, 0, 0];
    let mut num2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    let result = merge_sorted_array(&mut num1, m, &mut num2, n);
    println!("{:?}", result);
}