use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::u32;

// prim.rs --- Prim
// author: Seong Yong-ju <sei40kr@gmail.com>

struct Edge {
    to: usize,
    cost: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    weight: u32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn prim(edges: &Vec<Vec<Edge>>) -> u32 {
    let n = edges.len();

    let mut visited = vec![false; n];

    let mut queue = BinaryHeap::<State>::new();
    queue.push(State { node: 0, weight: 0 });

    let mut total_cost = 0;

    while let Some(State { node, weight }) = queue.pop() {
        if visited[node] {
            continue;
        }

        visited[node] = true;

        total_cost += weight;

        for adj in edges[node].iter() {
            queue.push(State {
                node: adj.to,
                weight: adj.cost,
            });
        }
    }

    return total_cost;
}
