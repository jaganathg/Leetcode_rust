fn find_it_odd(arr: &[i32]) -> i32 {
    let mut _count = 0;
    for &num in arr {
        _count += arr.iter().filter(|&x| *x == num).count() % 2;
    }
    arr.iter().find(|&&x| arr.iter().filter(|&y| *y == x).count() % 2 == 1).unwrap().to_owned()
    
}


fn main() {
    let arr = [1, 1, 2, 2, 3, 3, 3];
    let result = find_it_odd(&arr);
    println!("{:?}", result);
}