


fn count_complete_component(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut graph = vec![vec![]; n as usize];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
        graph[edge[1] as usize].push(edge[0] as usize);
    }
    let mut visited = vec![false; n as usize];
    let mut count = 0;
    for i in 0..n {
        if !visited[i as usize] {
            let mut nodes = vec![];
            dfs(i as usize, &mut  visited, &graph, &mut nodes);
            if is_complete_components(&nodes, &graph) {
                count += 1;
            }
        }
    }
    count
}

fn dfs(i: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, node: &mut Vec<usize>) {
    visited[i] = true;
    node.push(i);
    for &j in &graph[i] {
        if !visited[j] {
            dfs(j, visited, graph, node);
        }
    }
}

fn is_complete_components(nodes: &Vec<usize>, graph: &Vec<Vec<usize>>) -> bool {
    let size = nodes.len();

    for &node in nodes {
        if graph[node].len() != size - 1 {
            return false;
        }
        for &neighbor in &graph[node] {
            if !nodes.contains(&neighbor) {
                return false;
            }
        
        }
    }
    true
}

fn main() {
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    let res = count_complete_component(n, edges);
    println!("{}", res);
}