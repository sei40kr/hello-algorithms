use std::collections::VecDeque;

// kahns-topological-sort.rs --- Kahn's Topological sort
// author: Seong Yong-ju <sei40kr@gmail.com>

struct Edge {
    to: usize,
}

#[allow(dead_code)]
fn topological_sort(edges: &Vec<Vec<Edge>>) -> Option<Vec<usize>> {
    let n = edges.len();

    // Create a vector to store indegrees of all vertices. Initialize all
    // indegrees as 0.
    let mut in_degrees = vec![0; n];

    // Traverse adjacency lists to fill indegrees of vertices
    for adjs in edges.iter() {
        for adj in adjs.iter() {
            in_degrees[adj.to] += 1;
        }
    }

    // Create an queue and enqueue all vertices with indegree 0
    let mut queue = VecDeque::<usize>::new();

    for (i, &in_degree) in in_degrees.iter().enumerate() {
        if in_degree == 0 {
            queue.push_back(i);
        }
    }

    // Create a vector to store result
    let mut result = Vec::with_capacity(n);
    // Initialize count of visited vertices
    let mut count = 0;

    // One by one dequeue vertices from queue and enqueue adjacents if indegree
    // of adjacents becomes 0
    while let Some(i) = queue.pop_front() {
        result.push(i);

        // Iterate through all its neighbouring nodes of dequeued node i
        // and decrease their in-degree by 1
        for adj in edges[i].iter() {
            in_degrees[adj.to] -= 1;

            // If in-degree becomes zero, add it to queue
            if in_degrees[adj.to] == 0 {
                queue.push_back(adj.to);
            }
        }

        count += 1;
    }

    // Check if there was a cycle
    if count != n {
        None
    } else {
        Some(result)
    }
}
