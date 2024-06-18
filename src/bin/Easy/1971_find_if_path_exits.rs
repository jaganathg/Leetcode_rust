use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<i32, HashSet<i32>>;


fn add_edges(graph: &mut Graph, u: i32, v: i32) {
    graph.entry(u).or_insert(HashSet::new()).insert(v);
    graph.entry(v).or_insert(HashSet::new()).insert(u);
}

fn find_if_path_exists(_n: i32, edges: Vec<Vec<i32>>, src: i32, des: i32) -> bool {

        let mut graph: Graph = HashMap::new();

        for edge in edges {
            add_edges(&mut graph, edge[0], edge[1]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(src);
        visited.insert(src);

        while let Some(node) = queue.pop_front() {
            if node == des {
                return true;
            }

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }
        }
    false
}
    

fn main() {
    let n = 6;
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 3],
        vec![2, 3],
        // vec![3, 4],
        vec![4, 5],
    ];
    let source = 0;
    let destination = 5;

    let result = find_if_path_exists(n, edges, source, destination);
    println!("Is there a valid path from {} to {}? {}", source, destination, result);
}
