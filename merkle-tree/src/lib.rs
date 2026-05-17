use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn hash_leaf(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u8(0x00);
    hasher.write(data);
    hasher.finish()
}

fn hash_node(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u8(0x01);
    hasher.write_u64(left);
    hasher.write_u64(right);
    hasher.finish()
}

/// Merkle Tree implementation.
/// Leaves are hashed individually, and each internal node is the hash of the
/// concatenation of its two children. When a level has an odd number of nodes,
/// the last node is duplicated to pair with itself.
///
/// # Example
///
/// ```
/// let leaves = [b"a".as_slice(), b"b", b"c", b"d"];
/// let tree = merkletree::MerkleTree::new(&leaves);
/// let root = tree.root().unwrap();
/// let proof = tree.proof(2).unwrap();
/// assert!(merkletree::MerkleTree::verify(&b"c".as_slice(), &proof, root));
/// ```
pub struct MerkleTree {
    levels: Vec<Vec<u64>>,
}

impl MerkleTree {
    /// Builds a Merkle tree from the given leaves.
    ///
    /// # Arguments
    ///
    /// * `leaves` - The leaf values to build the tree from
    pub fn new<T: AsRef<[u8]>>(leaves: &[T]) -> Self {
        if leaves.is_empty() {
            return MerkleTree { levels: vec![] };
        }

        let mut current: Vec<u64> = leaves.iter().map(|l| hash_leaf(l.as_ref())).collect();
        let mut levels = vec![current.clone()];

        while current.len() > 1 {
            let mut next = Vec::with_capacity((current.len() + 1) / 2);
            for chunk in current.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() == 2 { chunk[1] } else { chunk[0] };
                next.push(hash_node(left, right));
            }
            current = next;
            levels.push(current.clone());
        }

        MerkleTree { levels }
    }

    /// Returns the Merkle root hash, or `None` if the tree is empty.
    pub fn root(&self) -> Option<u64> {
        self.levels.last().and_then(|level| level.first().copied())
    }

    /// Returns an inclusion proof for the leaf at the given index.
    /// Each entry is the sibling hash along with a flag indicating whether the
    /// sibling sits to the right of the current hash.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the leaf to prove
    pub fn proof(&self, mut index: usize) -> Option<Vec<(u64, bool)>> {
        if self.levels.is_empty() || index >= self.levels[0].len() {
            return None;
        }

        let mut proof = Vec::with_capacity(self.levels.len() - 1);
        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_index = index ^ 1;
            let sibling = if sibling_index < level.len() {
                level[sibling_index]
            } else {
                level[index]
            };
            let sibling_is_right = index % 2 == 0;
            proof.push((sibling, sibling_is_right));
            index /= 2;
        }

        Some(proof)
    }

    /// Verifies that `leaf` is included under `root` given the supplied proof.
    pub fn verify<T: AsRef<[u8]>>(leaf: &T, proof: &[(u64, bool)], root: u64) -> bool {
        let mut hash = hash_leaf(leaf.as_ref());
        for &(sibling, sibling_is_right) in proof {
            hash = if sibling_is_right {
                hash_node(hash, sibling)
            } else {
                hash_node(sibling, hash)
            };
        }
        hash == root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let leaves: [&[u8]; 0] = [];
        let tree = MerkleTree::new(&leaves);
        assert_eq!(tree.root(), None);
        assert_eq!(tree.proof(0), None);
    }

    #[test]
    fn test_single_leaf() {
        let leaves = [b"hello".as_slice()];
        let tree = MerkleTree::new(&leaves);
        assert_eq!(tree.root(), Some(hash_leaf(b"hello")));
        assert_eq!(tree.proof(0), Some(vec![]));
        assert!(MerkleTree::verify(
            &b"hello".as_slice(),
            &[],
            tree.root().unwrap()
        ));
    }

    #[test]
    fn test_root_is_deterministic() {
        let leaves = [b"a".as_slice(), b"b", b"c", b"d"];
        let tree1 = MerkleTree::new(&leaves);
        let tree2 = MerkleTree::new(&leaves);
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_proof_verifies_for_every_leaf() {
        let leaves = [b"a".as_slice(), b"b", b"c", b"d", b"e", b"f", b"g", b"h"];
        let tree = MerkleTree::new(&leaves);
        let root = tree.root().unwrap();

        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof(i).unwrap();
            assert!(MerkleTree::verify(leaf, &proof, root));
        }
    }

    #[test]
    fn test_proof_verifies_with_odd_number_of_leaves() {
        let leaves = [b"a".as_slice(), b"b", b"c", b"d", b"e"];
        let tree = MerkleTree::new(&leaves);
        let root = tree.root().unwrap();

        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof(i).unwrap();
            assert!(MerkleTree::verify(leaf, &proof, root));
        }
    }

    #[test]
    fn test_tampered_leaf_fails_verification() {
        let leaves = [b"a".as_slice(), b"b", b"c", b"d"];
        let tree = MerkleTree::new(&leaves);
        let root = tree.root().unwrap();

        let proof = tree.proof(1).unwrap();
        assert!(!MerkleTree::verify(&b"x".as_slice(), &proof, root));
    }

    #[test]
    fn test_tampered_root_fails_verification() {
        let leaves = [b"a".as_slice(), b"b", b"c", b"d"];
        let tree = MerkleTree::new(&leaves);
        let proof = tree.proof(1).unwrap();
        assert!(!MerkleTree::verify(&b"b".as_slice(), &proof, 0));
    }

    #[test]
    fn test_out_of_range_proof_returns_none() {
        let leaves = [b"a".as_slice(), b"b", b"c"];
        let tree = MerkleTree::new(&leaves);
        assert!(tree.proof(3).is_none());
    }
}
