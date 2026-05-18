use std::collections::VecDeque;

/// A directed graph represented as an adjacency list.
///
/// # Example
///
/// ```
/// let mut g = topological_sort::Graph::new(4);
/// g.add_edge(0, 1);
/// g.add_edge(0, 2);
/// g.add_edge(1, 3);
/// g.add_edge(2, 3);
/// let order = g.kahn().unwrap();
/// assert_eq!(order[0], 0);
/// assert_eq!(order[3], 3);
/// ```
pub struct Graph {
    adjacency: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates a new graph with the given number of vertices and no edges.
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            adjacency: vec![vec![]; num_vertices],
        }
    }

    /// Adds a directed edge `from -> to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency[from].push(to);
    }

    /// Returns a topological ordering of the vertices using Kahn's algorithm
    /// (BFS over zero-indegree vertices), or `None` if the graph has a cycle.
    pub fn kahn(&self) -> Option<Vec<usize>> {
        let n = self.adjacency.len();
        let mut in_degree = vec![0usize; n];
        for adj in &self.adjacency {
            for &v in adj {
                in_degree[v] += 1;
            }
        }

        let mut queue: VecDeque<usize> =
            (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::with_capacity(n);

        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &self.adjacency[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        // `in_degree[v]` is only decremented when a predecessor of `v` is
        // popped — so it reaches 0 only after every predecessor has been
        // emitted. For a vertex on a cycle u -> ... -> v -> ... -> u, emitting
        // u would require v's predecessor on the cycle to be emitted first,
        // which transitively requires u itself. None of them can ever be
        // popped, and the vertices missing from `order` are exactly the ones
        // trapped in (or reachable only through) a cycle.
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }

    /// Returns a topological ordering of the vertices using a DFS with
    /// three-state marking, or `None` if the graph has a cycle.
    pub fn dfs(&self) -> Option<Vec<usize>> {
        let n = self.adjacency.len();
        let mut state = vec![VisitState::Unvisited; n];
        let mut order = Vec::with_capacity(n);

        for i in 0..n {
            if state[i] == VisitState::Unvisited
                && !Self::visit(&self.adjacency, &mut state, &mut order, i)
            {
                return None;
            }
        }

        order.reverse();
        Some(order)
    }

    fn visit(
        adjacency: &[Vec<usize>],
        state: &mut [VisitState],
        order: &mut Vec<usize>,
        u: usize,
    ) -> bool {
        match state[u] {
            VisitState::Finished => return true,
            // Found a back edge: the graph has a cycle.
            VisitState::InProgress => return false,
            VisitState::Unvisited => {}
        }

        state[u] = VisitState::InProgress;
        for &v in &adjacency[u] {
            if !Self::visit(adjacency, state, order, v) {
                return false;
            }
        }
        state[u] = VisitState::Finished;
        order.push(u);
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Unvisited,
    InProgress,
    Finished,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_topological_order(graph: &Graph, order: &[usize]) -> bool {
        let n = graph.adjacency.len();
        if order.len() != n {
            return false;
        }
        let mut position = vec![usize::MAX; n];
        for (i, &v) in order.iter().enumerate() {
            if position[v] != usize::MAX {
                return false;
            }
            position[v] = i;
        }
        for (u, adj) in graph.adjacency.iter().enumerate() {
            for &v in adj {
                if position[u] >= position[v] {
                    return false;
                }
            }
        }
        true
    }

    fn linear_dag() -> Graph {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g
    }

    fn diamond_dag() -> Graph {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        g
    }

    fn course_dag() -> Graph {
        // 0: intro, 1: data structures, 2: algorithms,
        // 3: databases, 4: distributed systems
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);
        g.add_edge(3, 4);
        g
    }

    fn cyclic_graph() -> Graph {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        g
    }

    #[test]
    fn test_kahn_linear() {
        let g = linear_dag();
        assert_eq!(g.kahn(), Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_dfs_linear() {
        let g = linear_dag();
        assert_eq!(g.dfs(), Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_kahn_diamond_is_valid_order() {
        let g = diamond_dag();
        let order = g.kahn().unwrap();
        assert!(is_valid_topological_order(&g, &order));
    }

    #[test]
    fn test_dfs_diamond_is_valid_order() {
        let g = diamond_dag();
        let order = g.dfs().unwrap();
        assert!(is_valid_topological_order(&g, &order));
    }

    #[test]
    fn test_kahn_course_dag_is_valid_order() {
        let g = course_dag();
        let order = g.kahn().unwrap();
        assert!(is_valid_topological_order(&g, &order));
    }

    #[test]
    fn test_dfs_course_dag_is_valid_order() {
        let g = course_dag();
        let order = g.dfs().unwrap();
        assert!(is_valid_topological_order(&g, &order));
    }

    #[test]
    fn test_kahn_detects_cycle() {
        let g = cyclic_graph();
        assert_eq!(g.kahn(), None);
    }

    #[test]
    fn test_dfs_detects_cycle() {
        let g = cyclic_graph();
        assert_eq!(g.dfs(), None);
    }

    #[test]
    fn test_kahn_detects_self_loop() {
        let mut g = Graph::new(2);
        g.add_edge(0, 1);
        g.add_edge(1, 1);
        assert_eq!(g.kahn(), None);
    }

    #[test]
    fn test_dfs_detects_self_loop() {
        let mut g = Graph::new(2);
        g.add_edge(0, 1);
        g.add_edge(1, 1);
        assert_eq!(g.dfs(), None);
    }

    #[test]
    fn test_empty_graph() {
        let g = Graph::new(0);
        assert_eq!(g.kahn(), Some(vec![]));
        assert_eq!(g.dfs(), Some(vec![]));
    }

    #[test]
    fn test_disconnected_graph() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        let kahn_order = g.kahn().unwrap();
        let dfs_order = g.dfs().unwrap();
        assert!(is_valid_topological_order(&g, &kahn_order));
        assert!(is_valid_topological_order(&g, &dfs_order));
    }
}
