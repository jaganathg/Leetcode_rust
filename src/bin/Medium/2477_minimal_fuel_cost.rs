
fn minimal_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut graph = vec![vec![]; roads.len() + 1 as usize];
    for road in roads {
        graph[road[0] as usize].push(road[1] as usize);
        graph[road[1] as usize].push(road[0] as usize);
    }

    let mut result: usize = 0;

    dfs(0, -1, &graph, seats as usize, &mut result);
    
    result as i64
}

fn dfs(node: usize, parent: isize, graph: &Vec<Vec<usize>>, seat: usize, result: &mut usize) -> usize  {
    let mut representative = 1;

    for &neighbor in &graph[node] {
        if neighbor as isize == parent {
            continue;
        }
        let sub_representative = dfs(neighbor, node as isize, graph, seat, result);
        representative += sub_representative;

        *result += (sub_representative + seat -1)/ seat;

    }
    representative
}


fn main() {
    let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let seats = 5;
    let res = minimal_fuel_cost(roads, seats);
    println!("{}", res);
}


//------------------------------------------------------------------------------
/*
use std::ops::AddAssign;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut adjacency_list = vec![vec![]; roads.len() + 1];
        for road in roads.into_iter() {
            let (city_a, city_b) = (road[0], road[1]);
            adjacency_list[city_a as usize].push(city_b);
            adjacency_list[city_b as usize].push(city_a);
        }

        #[derive(Debug)]
        struct DfsResult {
            travellers: i32,
            fuel_consumed: i64,
        }
        impl AddAssign<DfsResult> for DfsResult {
            fn add_assign(&mut self, rhs: DfsResult) {
                self.travellers += rhs.travellers;
                self.fuel_consumed += rhs.fuel_consumed;
            }
        }
        fn dfs(index: i32, parent_index: Option<i32>, adjacency_list: &Vec<Vec<i32>>, seats: i32) -> DfsResult {
            if parent_index.is_some() && adjacency_list[index as usize].len() == 1 {
                return DfsResult {
                    travellers: 1,
                    fuel_consumed: 0,
                };
            }

            adjacency_list[index as usize].iter()
                .copied()
                .filter(|i| Some(*i) != parent_index)
                .map(|i| dfs(i, Some(index), adjacency_list, seats))
                .map(|mut r| {
                    r.fuel_consumed += (r.travellers as i64 + seats as i64 - 1) / seats as i64;
                    r
                })
                .fold(DfsResult {
                    travellers: 1,
                    fuel_consumed: 0,
                }, |mut a, b| {
                    a += b;
                    a
                })
        }

        let r = dfs(0, None, &adjacency_list, seats);
        r.fuel_consumed
    }
}

*/