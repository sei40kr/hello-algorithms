// fenwick-tree.rs --- Fenwick Tree implementation
// author: Seong Yong-ju <sei40kr@gmail.com>

struct FenwickTree {
    n: usize,
    data: Vec<usize>,
}

#[allow(dead_code)]
impl FenwickTree {
    fn new(size: usize) -> FenwickTree {
        FenwickTree {
            n: size + 1,
            data: vec![0; size + 1],
        }
    }

    fn add(&mut self, k: usize, value: usize) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= x + 1;
        }
    }

    // Returns a sum of range [l, r)
    fn sum(&self, l: usize, r: usize) -> usize {
        self.sum_one(r) - self.sum_one(l)
    }

    // Returns a sum of range [0, k)
    fn sum_one(&self, k: usize) -> usize {
        assert!(k < self.n);

        let mut sum = 0;
        let mut x = k as i32 - 1;
        while 0 <= x {
            sum += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        sum
    }
}
