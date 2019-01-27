use std::env;

// kruskal.rs --- Kruskal
// author: Seong Yong-ju <sei40kr@gmail.com>

struct Edge {
    from: usize,
    to: usize,
    cost: u32,
}

// kenkoooo's Union-Find implementation

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        return true;
    }
}

#[allow(dead_code)]
fn kruskal(v: usize, edges: &mut Vec<Edge>) -> u32 {
    edges.sort_by_key(|edge| edge.cost);
    let mut uf = UnionFind::new(v);
    let mut result = 0;

    for edge in edges.iter() {
        if uf.find(edge.from) != uf.find(edge.to) {
            uf.unite(edge.from, edge.to);
            result += edge.cost;
        }
    }

    result
}
