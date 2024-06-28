
fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut table = vec![0; n as usize];
    let mut res = 0;

    for road in roads.iter() {
        let u = road[0] as usize;
        let v = road[1] as usize;
        table[u] += 1;
        table[v] += 1;
    }

    table.sort();

    for (i, &count) in table.iter().enumerate() {
        res += (count as i64) * ((i + 1) as i64)
    }
    res
}

fn main() {
    let n = 5;
    let roads = vec![
        vec![0, 1], 
        vec![1, 2], 
        vec![2, 3], 
        vec![0, 2], 
        vec![1, 3], 
        vec![2, 4]
    ];
    let res = maximum_importance(n, roads);
    println!("{}", res);  // Output should match the expected result
}