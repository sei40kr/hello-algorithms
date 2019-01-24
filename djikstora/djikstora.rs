use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::u32;

struct Edge {
    from: usize,
    to: usize,
    cost: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn shortest_path(n: usize, s: usize, g: usize, edges: &Vec<Vec<Edge>>) -> Option<u32> {
    let mut dists: Vec<_> = vec![u32::MAX; n];
    dists[s] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(State { node: s, cost: 0 });

    while let Some(State { node, cost }) = queue.pop() {
        if node == g {
            return Some(cost);
        }

        if cost < dists[node] {
            dists[node] = cost;

            for edge in &edges[node] {
                queue.push(State {
                    node: edge.to,
                    cost: cost + edge.cost,
                });
            }
        }
    }

    None
}
