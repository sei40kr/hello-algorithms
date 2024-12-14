/// Implementation of the Union-Find (Disjoint Set Union) data structure.
/// Each element belongs to a set, and sets are managed using a tree structure.
/// Uses rank and path compression for efficient operations.
pub struct UnionFind {
    parents: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a new Union-Find structure.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of elements
    ///
    /// # Examples
    ///
    /// ```
    /// let mut uf = unionfind::UnionFind::new(10);
    /// ```
    pub fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    /// Finds the root of element `x`.
    ///
    /// Uses path compression to reduce the height of the tree.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the element
    ///
    /// # Returns
    ///
    /// The index of the root of `x`
    ///
    /// # Examples
    ///
    /// ```
    /// let mut uf = unionfind::UnionFind::new(10);
    /// let root = uf.find(3);
    /// ```
    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }

        self.parents[x] = self.find(self.parents[x]);
        self.parents[x]
    }

    /// Unites the sets that contain elements `x` and `y`.
    ///
    /// Uses rank to keep the tree height minimal.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of one element
    /// * `y` - The index of the other element
    ///
    /// # Examples
    ///
    /// ```
    /// let mut uf = unionfind::UnionFind::new(10);
    /// uf.union(3, 4);
    /// ```
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parents[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parents[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parents[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_initial_state() {
        let mut uf = UnionFind::new(10);
        for i in 0..10 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn test_union_find_union_and_find() {
        let mut uf = UnionFind::new(10);

        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(2, 4);

        assert_eq!(uf.find(1), uf.find(3));
        assert_eq!(uf.find(2), uf.find(4));
        assert_ne!(uf.find(1), uf.find(5));
    }

    #[test]
    fn test_union_find_multiple_unions() {
        let mut uf = UnionFind::new(10);

        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);
        uf.union(4, 5);

        for i in 1..6 {
            assert_eq!(uf.find(i), uf.find(1));
        }

        for i in 6..10 {
            assert_ne!(uf.find(i), uf.find(1));
        }
    }

    #[test]
    fn test_union_find_with_ranks() {
        let mut uf = UnionFind::new(10);

        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(5, 6);
        uf.union(7, 8);

        uf.union(1, 3);
        uf.union(5, 7);

        assert_eq!(uf.find(1), uf.find(4));
        assert_eq!(uf.find(5), uf.find(8));
        assert_ne!(uf.find(1), uf.find(5));

        uf.union(1, 5);

        for i in 1..9 {
            assert_eq!(uf.find(i), uf.find(1));
        }
    }

    #[test]
    fn test_union_find_self_union() {
        let mut uf = UnionFind::new(10);

        for i in 0..10 {
            uf.union(i, i);
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn test_union_find_large_scale() {
        let size = 1000;
        let mut uf = UnionFind::new(size);

        for i in 0..(size - 1) {
            uf.union(i, i + 1);
        }

        for i in 0..size {
            assert_eq!(uf.find(i), uf.find(0));
        }
    }
}
