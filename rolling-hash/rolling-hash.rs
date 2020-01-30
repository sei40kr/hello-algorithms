const BASE: i128 = 256;

struct RollingHash {
    hs: Vec<i128>,
    pw: Vec<i128>,
}

impl RollingHash {
    pub fn new(s: String) -> RollingHash {
        let mut hs = vec![0; s.len() + 1];
        let mut pw = vec![0; s.len() + 1];

        pw[0] = 1;

        for (i, c) in s.chars().enumerate() {
            hs[i + 1] = hs[i] * BASE + c as i128;
            pw[i + 1] = pw[i] * BASE;
        }

        RollingHash { hs, pw }
    }

    pub fn hash(&self, l: usize, r: usize) -> i128 {
        return self.hs[r + 1] - self.hs[l + 1] * self.pw[r - l];
    }

    pub fn equals(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        return self.hash(l1, r1) == self.hash(l2, r2);
    }
}

fn main() {
    let rh = RollingHash::new("foobarfoo".to_string());

    // foo == bar
    assert!(!rh.equals(0, 2, 3, 5));
    // foo == foo
    assert!(rh.equals(0, 2, 6, 8));
}
