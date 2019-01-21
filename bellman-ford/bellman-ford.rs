// bellman-ford.rs --- Bellman Ford
// author: Seong Yong-ju <sei40kr@gmail.com>

struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

#[allow(dead_code)]
fn shortest_path(n: usize, s: usize, edges: &Vec<Edge>) -> Vec<i32> {
    let mut dists = vec![std::i32::MAX; n];
    dists[s] = 0;

    loop {
        let mut update = false;

        for edge in edges.iter() {
            if dists[edge.from] != std::i32::MAX && dists[edge.from] + edge.cost < dists[edge.to] {
                dists[edge.to] = dists[edge.from] + edge.cost;
                update = true;
            }
        }

        if !update {
            break;
        }
    }

    return dists;
}

#[allow(dead_code)]
fn find_negative_loop(n: usize, edges: &Vec<Edge>) -> bool {
    let mut dists = vec![0; n];

    for i in 0..n {
        for edge in edges.iter() {
            if dists[edge.from] + edge.cost < dists[edge.to] {
                dists[edge.to] = dists[edge.from] + edge.cost;

                if i == n - 1 {
                    return true;
                }
            }
        }
    }

    return false;
}
