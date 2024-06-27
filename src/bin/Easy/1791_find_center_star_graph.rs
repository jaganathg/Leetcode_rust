

fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let val = edges[0][0];
    if edges[1].contains(&val) {
        val
    } else {
        edges[0][1]
    }
}

fn main() {
    // Example 1
    let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
    assert_eq!(find_center(edges), 2);

    // Example 2
    let edges = vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]];
    assert_eq!(find_center(edges), 1);

    println!("Success!");
}