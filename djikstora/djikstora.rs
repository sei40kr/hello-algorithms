use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::u32;

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: u32,
}

#[allow(dead_code)]
fn shortest_path(n: usize, s: usize, g: usize, edges_by_from: &Vec<Vec<Edge>>) -> Option<u32> {
    let mut dists: Vec<_> = vec![u32::MAX; n];
    dists[s] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(State { cost: 0, node: s });

    while let Some(State { node, cost }) = queue.pop() {
        if node == g {
            return Some(cost);
        }

        if cost < dists[node] {
            dists[node] = cost;

            for edge in &edges_by_from[node] {
                queue.push(State {
                    node: edge.node,
                    cost: cost + edge.cost,
                });
            }
        }
    }

    None
}
