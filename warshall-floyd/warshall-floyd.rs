// warshall-floyd.rs --- Warshall-Floyd
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
fn warshall_floyd(edges: usize, dists: &mut [&mut [i32]]) {
    for k in 0..edges {
        for i in 0..edges {
            for j in 0..edges {
                dists[i][j] = dists[i][j].min(dists[i][k] + dists[k][j]);
            }
        }
    }
}
