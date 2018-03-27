#include <algorithm>
#define REP(i, n) for (int i = 0; i < (n); ++i)

// union-find.cpp
// author: Seong Yong-ju <sei40kr@gmail.com>

// Union-Find 木の実装
// cf https://www.slideshare.net/chokudai/union-find-49066733
const int MAX_N = 2014;

int par[MAX_N];

void init(int n) {
  REP(i, n) { par[i] = i; }
}

int root(int x) {
  return par[x] == x ? x : par[x] = root(par[x]);
}

bool same(int x, int y) {
  return root(x) == root(y);
}

void unite(int x, int y) {
  x = root(x);
  y = root(y);
  if (x != y) {
    par[x] = y;
  }
}
