#include <cstdlib>

// binary-search.cpp --- Binary search implementation
// author: Seong Yong-ju <sei40kr@gmail.com>

int solve(int x) {
  // TODO Implement this
  return true;
}

int main() {
  int ok = 0;
  int ng = 1000;

  while (abs(ok - ng) > 1) {
    int mid = (ok + ng) / 2;

    if (solve(mid)) {
      ok = mid;
    } else {
      ng = mid;
    }
  }

  return 0;
}
