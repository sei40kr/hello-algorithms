/// Fenwick Tree (Binary Indexed Tree) implementation.
/// This data structure allows efficient prefix sum queries and updates.
///
/// # Example
///
/// ```
/// let mut fenwick_tree = fenwicktree::FenwickTree::new(10);
/// fenwick_tree.update(3, 5);
/// fenwick_tree.update(5, 2);
/// assert_eq!(fenwick_tree.query(5), 7); // 5 (at index 3) + 2 (at index 5)
/// assert_eq!(fenwick_tree.query(3), 5); // 5 (at index 3)
/// ```
pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree with the given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of elements
    ///
    /// # Examples
    ///
    /// ```
    /// let fenwick_tree = fenwicktree::FenwickTree::new(10);
    /// ```
    pub fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
        }
    }

    /// Updates the value at the given index by the given delta.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to update (1-based index)
    /// * `delta` - The value to add to the index
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fenwick_tree = fenwicktree::FenwickTree::new(10);
    /// fenwick_tree.update(3, 5);
    /// ```
    pub fn update(&mut self, mut index: usize, delta: i64) {
        while index < self.tree.len() {
            self.tree[index] += delta;
            index += index & (!index + 1);
        }
    }

    /// Queries the prefix sum from index 1 to the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to query up to (1-based index)
    ///
    /// # Returns
    ///
    /// The prefix sum from index 1 to the given index
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fenwick_tree = fenwicktree::FenwickTree::new(10);
    /// fenwick_tree.update(3, 5);
    /// fenwick_tree.update(5, 2);
    /// assert_eq!(fenwick_tree.query(5), 7);
    /// assert_eq!(fenwick_tree.query(3), 5);
    /// ```
    pub fn query(&self, mut index: usize) -> i64 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let mut fenwick_tree = FenwickTree::new(10);

        fenwick_tree.update(3, 5);
        fenwick_tree.update(5, 2);

        assert_eq!(fenwick_tree.query(5), 7); // 5 (at index 3) + 2 (at index 5)
        assert_eq!(fenwick_tree.query(3), 5); // 5 (at index 3)
        assert_eq!(fenwick_tree.query(4), 5); // 5 (at index 3)
        assert_eq!(fenwick_tree.query(6), 7); // 5 (at index 3) + 2 (at index 5)

        fenwick_tree.update(1, 3);
        assert_eq!(fenwick_tree.query(1), 3); // 3 (at index 1)
        assert_eq!(fenwick_tree.query(3), 8); // 3 (at index 1) + 5 (at index 3)
    }

    #[test]
    fn test_fenwick_tree_large_scale() {
        let size = 1000;
        let mut fenwick_tree = FenwickTree::new(size);

        for i in 1..=size {
            fenwick_tree.update(i, i as i64);
        }

        assert_eq!(fenwick_tree.query(1000), (1000 * 1001 / 2) as i64); // Sum of first 1000 natural numbers
    }
}
