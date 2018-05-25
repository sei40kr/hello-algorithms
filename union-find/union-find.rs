// union-find.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let new_par = self.root(par);
            self.par[x] = new_par;
            new_par
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.root(x);
        y = self.root(y);
        if x != y {
            self.par[x] = y;
        }
    }
}
