use std::cmp::min;
use std::collections::VecDeque;

/// A structure to represent a graph
pub struct Graph {
    pub capacity: Vec<Vec<i32>>,
    pub num_vertices: usize,
}

impl Graph {
    /// Creates a new graph with the given number of vertices
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            capacity: vec![vec![0; num_vertices]; num_vertices],
            num_vertices,
        }
    }

    /// Adds an edge with the given capacity to the graph
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        self.capacity[from][to] = capacity;
    }

    /// Implements the Edmonds-Karp algorithm to find the maximum flow
    pub fn edmonds_karp(&self, source: usize, sink: usize) -> i32 {
        let mut flow = 0;
        let mut residual_capacity = self.capacity.clone();
        let mut parent = vec![-1; self.num_vertices];

        while self.bfs(source, sink, &mut parent, &residual_capacity) {
            let mut path_flow = i32::MAX;
            let mut s = sink;

            while s != source {
                let p = parent[s] as usize;
                path_flow = min(path_flow, residual_capacity[p][s]);
                s = p;
            }

            flow += path_flow;
            let mut v = sink;

            while v != source {
                let u = parent[v] as usize;
                residual_capacity[u][v] -= path_flow;
                residual_capacity[v][u] += path_flow;
                v = u;
            }
        }

        flow
    }

    /// Helper function to perform BFS and find if there is a path from source to sink
    fn bfs(
        &self,
        source: usize,
        sink: usize,
        parent: &mut Vec<i32>,
        residual_capacity: &Vec<Vec<i32>>,
    ) -> bool {
        let mut visited = vec![false; self.num_vertices];
        let mut queue = VecDeque::new();
        queue.push_back(source);
        visited[source] = true;
        parent[source] = -1;

        while let Some(u) = queue.pop_front() {
            for v in 0..self.num_vertices {
                if !visited[v] && residual_capacity[u][v] > 0 {
                    queue.push_back(v);
                    parent[v] = u as i32;
                    visited[v] = true;
                }
            }
        }

        visited[sink]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_flow() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1, 16);
        graph.add_edge(0, 2, 13);
        graph.add_edge(1, 2, 10);
        graph.add_edge(1, 3, 12);
        graph.add_edge(2, 1, 4);
        graph.add_edge(2, 4, 14);
        graph.add_edge(3, 2, 9);
        graph.add_edge(3, 5, 20);
        graph.add_edge(4, 3, 7);
        graph.add_edge(4, 5, 4);

        assert_eq!(graph.edmonds_karp(0, 5), 23);
    }
}
