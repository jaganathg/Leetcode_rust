
use std::collections::{HashSet, HashMap};

fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Convert n to usize
    let n = n as usize;
    
    // Step 1: Represent the graph
    let mut graph = vec![Vec::new(); n];
    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[v].push(u);
    }

    // Step 2: Function for DFS
    fn dfs(node: usize, visited: &mut HashSet<usize>, ancestors: &mut HashSet<usize>, graph: &Vec<Vec<usize>>) {
        for &neighbor in &graph[node] {
            if visited.insert(neighbor) {
                ancestors.insert(neighbor);
                dfs(neighbor, visited, ancestors, graph);
            }
        }
    }

    // Step 3: Calculate ancestors for each node
    let mut res = vec![Vec::new(); n];
    for i in 0..n {
        let mut visited = HashSet::new();
        let mut ancestors = HashSet::new();
        dfs(i, &mut visited, &mut ancestors, &graph);
        let mut ancestor_list: Vec<i32> = ancestors.into_iter().map(|x| x as i32).collect();
        ancestor_list.sort();
        res[i] = ancestor_list;
    }

    res
}


fn get_ancestors_alternate(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;

    let mut graph = vec![vec![]; n];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    let mut ancestors = vec![vec![]; n];
    for ancestor in 0..n {
        let mut stack = graph[ancestor].clone();
        while let Some(node) = stack.pop() {
            if ancestors[node].last()
                .is_some_and(|&node| node == ancestor as i32)
            {
                continue;
            }

            ancestors[node].push(ancestor as i32);
            stack.extend_from_slice(&graph[node]);
        }
    }

    ancestors
}

fn main() {
    // Example 1
    let n = 8;
    let edges = vec![
        vec![0, 3], vec![0, 4], vec![1, 3], vec![2, 4], 
        vec![2, 7], vec![3, 5], vec![3, 6], vec![3, 7], 
        vec![4, 6]
    ];
    let result = get_ancestors(n, edges);
    for (i, ancestors) in result.iter().enumerate() {
        println!("Node {}: {:?}", i, ancestors);
    }

    // Example 2
    let n = 5;
    let edges = vec![
        vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], 
        vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], 
        vec![2, 4], vec![3, 4]
    ];
    let result = get_ancestors_alternate(n, edges);
    for (i, ancestors) in result.iter().enumerate() {
        println!("Node {}: {:?}", i, ancestors);
    }
}