# Merkle Tree

A Merkle tree is a binary tree in which each leaf stores the hash of a piece of data, and each internal node stores the hash of the concatenation of its two children. The single hash at the top — the **root hash** — changes whenever any leaf changes, so the structure is widely used for **tamper detection** and **inclusion proofs**. It shows up in blockchains, distributed storage, and Git's object model, anywhere data integrity needs to be verified efficiently.

## Notation

- $H(x)$ — the output of a hash function applied to $x$.
- $x \mathbin\Vert y$ — byte-string **concatenation** of $x$ followed by $y$. So $H(H(a) \mathbin\Vert H(b))$ means "concatenate the bytes of $H(a)$ and $H(b)$, then hash the result." This is standard cryptographic notation, used in preference to $+$ to avoid suggesting numeric addition.

## How the Tree Is Built

Starting from leaves $[a, b, c, d]$, hashes are combined upward level by level. **The table is laid out like the tree itself: the top row is the root and the bottom row is the leaves.** Level numbering starts at $0$ on the leaves and increases as we move up.

| Level     | Node count | Contents                                                             |
| --------- | ---------- | -------------------------------------------------------------------- |
| Level 2 (root) | 1          | $H(\text{Level1}[0] \mathbin\Vert \text{Level1}[1])$ ← **root hash** |
| Level 1   | 2          | $H(H(a) \mathbin\Vert H(b))$, $H(H(c) \mathbin\Vert H(d))$           |
| Level 0 (leaves) | 4          | $H(a)$, $H(b)$, $H(c)$, $H(d)$                                |

When a level has an odd number of nodes, the last node is duplicated and paired with itself.

## Inclusion Proof

### What it actually proves

An inclusion proof proves the statement **"the leaf at position $i$ is $x$, under root $r$."** Note that it does *not* prove "$x$ is somewhere in the tree" by itself — the position $i$ is part of the claim. A plain Merkle tree treats its leaves as an **ordered list**, not a set.

### How it works

To prove that the leaf at index $i$ is $x$, the prover reveals the **sibling hash** at each level along the path from that leaf up to the root. The verifier rehashes step by step from $H(x)$ and checks that the final value equals the root.

The proof for leaf $c$ (at index $2$) looks like this.

| Step | Current hash                            | Sibling hash                  | Sibling side | Next hash                                  |
| ---- | --------------------------------------- | ----------------------------- | ------------ | ------------------------------------------ |
| 1    | $H(c)$                                  | $H(d)$                        | right        | $H(H(c) \mathbin\Vert H(d))$               |
| 2    | $H(H(c) \mathbin\Vert H(d))$            | $H(H(a) \mathbin\Vert H(b))$  | left         | $H(L \mathbin\Vert R)$ = root hash         |

For $n$ leaves the tree height is $O(\log n)$, so proofs also fit in $O(\log n)$.

### Where does the index come from?

Because the proof is tied to a position, the natural question is: **how does anyone know the index of $x$?** A plain Merkle tree does not answer this — it is the caller's job. The signature in `src/lib.rs` reflects that:

```rust
pub fn proof(&self, index: usize) -> Option<Vec<(u64, bool)>>
```

In practice the index comes from the application:

| Use case                              | How the index is determined                                       |
| ------------------------------------- | ----------------------------------------------------------------- |
| Bitcoin / Ethereum transactions       | Natural ordering inside the block — Tx #42 *is* the 42nd Tx       |
| Certificate Transparency log          | Sequence number assigned on append                                |
| Git tree objects                      | Indexed by file path / fixed ordering                             |
| Arbitrary "is $x$ in the set?" query  | Not solvable with a plain Merkle tree — see the variants below    |

### What a plain Merkle tree does *not* give you

- **Set membership lookup**: "is $x$ anywhere in the tree?" — would require $O(n)$ scan, or an external `data → index` map.
- **Non-membership proofs**: "$x$ is *not* in the tree."

For those, use a variant where the position is derived from $x$ itself:

| Variant                       | Position rule              | Adds                                      |
| ----------------------------- | -------------------------- | ----------------------------------------- |
| **Sparse Merkle Tree**        | $\text{index} = H(x)$      | Verifiable membership *and* non-membership |
| **Merkle Patricia Trie** (Ethereum state) | Key nibbles form the path  | Compressed sparse trie over key→value     |
| **Verkle Tree**               | Same idea, vector commitment instead of binary hash | Much shorter proofs       |

### Why this is still useful

The plain construction is the right tool when leaves have a natural position (block, log, file tree). The point of the inclusion proof is **succinct verification against a tiny commitment** — the verifier holds one root hash and accepts $O(\log n)$ hashes from an untrusted prover. Searching for the index is the prover's problem; the verifier never searches.

## Complexity

| Operation         | Time          | Space         |
| ----------------- | ------------- | ------------- |
| Build (`new`)     | $O(n)$        | $O(n)$        |
| Root (`root`)     | $O(1)$        | -             |
| Proof (`proof`)   | $O(\log n)$   | $O(\log n)$   |
| Verify (`verify`) | $O(\log n)$   | -             |

## API

| Method                                  | Description                                              |
| --------------------------------------- | -------------------------------------------------------- |
| `MerkleTree::new(leaves)`               | Build a tree from a slice of leaves                      |
| `tree.root()`                           | Return the root hash (`None` if the tree is empty)       |
| `tree.proof(index)`                     | Return an inclusion proof for the leaf at `index`        |
| `MerkleTree::verify(leaf, proof, root)` | Check that `leaf` is included under `root` via the proof |

## References

- [Merkle tree - Wikipedia](https://en.wikipedia.org/wiki/Merkle_tree)
