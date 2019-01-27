// dfs-topological-sort.rs --- Topological sort
// author: Seong Yong-ju <sei40kr@gmail.com>

struct Edge {
    to: usize,
}

fn visit(edges: &Vec<Vec<Edge>>, colors: &mut Vec<u32>, i: usize, result: &mut Vec<usize>) -> bool {
    if colors[i] == 2 {
        return true;
    }
    if colors[i] == 1 {
        return false;
    }

    colors[i] = 1;

    for adj in edges[i].iter() {
        if !visit(edges, colors, adj.to, result) {
            return false;
        }
    }

    colors[i] = 2;

    result.push(i);
    return true;
}

#[allow(dead_code)]
fn topological_sort(edges: &Vec<Vec<Edge>>) -> Option<Vec<usize>> {
    let n = edges.len();

    // Mark all the vertices as not visited
    let mut colors = vec![0; n];

    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        if !visit(edges, &mut colors, i, &mut result) {
            return None;
        }
    }

    result.reverse();
    return Some(result);
}
