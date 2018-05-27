#include <climits>
#include <iostream>
#include <string>

#define REP(i, n) for (int i = 0; i < (n); i++)

// segment-tree.cpp --- Segment tree implementation
// author: Seong Yong-ju <sei40kr@gmail.com>
//
// Note that this implementation works only where N is the power of 2.

using namespace std;

const int MAX_N = 131072;

/// Initialize segment-tree as an array.
//
// The parent of the nth element: floor((n-1)/2)
// The children of the nth element: 2*n-1, 2*n+2

int n;
int dat[MAX_N * 2 - 1];

void init() {
  REP(i, 2 * n - 1) { dat[i] = INT_MAX; }
}

// In this case, returns the less one.
int compare(int l, int r) { return min(l, r); }

void update(int i, int x) {
  // Transform the index
  i += n - 1;
  // Update the value
  dat[i] = x;

  while (0 < i) {
    i = (i - 1) / 2;
    // Update the parents with the less one
    dat[i] = compare(dat[i * 2 + 1], dat[i * 2 + 2]);
  }
}

// Returns the minimum value in [a, b).
int query(int a, int b, int k, int l, int r) {
  if (r <= a || b <= l) {
    return INT_MAX;
  }
  if (a <= l && r <= b) {
    return dat[k];
  } else {
    int vl = query(a, b, k * 2 + 1, l, (l + r) / 2);
    int vr = query(a, b, k * 2 + 2, (l + r) / 2, r);

    // Return the less one
    return compare(vl, vr);
  }
}

int main() { return 0; }
