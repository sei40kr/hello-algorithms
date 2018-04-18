#include <cmath>
#include <climits>
#include <algorithm>

// segment-tree2.cpp --- Segment tree implementation 2
// author: Seong Yong-ju <sei40kr@gmail.com>

using namespace std;

const int MAX_N = 131072;

struct segtree {
  int N, dat[2 * MAX_N];

  segtree() {}

  explicit segtree(int n) {
    N = static_cast<int>(ceil(log2(n)));
    fill(dat, dat + (2 * N - 1), INT_MAX);
  }

  // Query for the minimum value in [a, b)
  int query(int a, int b) { return query(a, b, 0, 0, N); }

  int query(int a, int b, int k, int l, int r) {
    if (r <= a || b <= l) {
      return INT_MAX;
    }
    if (a <= l && r <= b) {
      return dat[k];
    }
    int m = (l + r) / 2;
    return min(query(a, b, k * 2 + 1, l, m), query(a, b, k * 2 + 2, m, r));
  }
};
