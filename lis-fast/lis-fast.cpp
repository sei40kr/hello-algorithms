#include <bits/stdc++.h>

// lis-fast.cpp
// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

long lis(const vector<int> &vec) {
  size_t l = vec.size();
  vector<long> dp(l, LONG_MAX);
  REP(i, l) { *lower_bound(dp.begin(), dp.end(), vec[i]) = vec[i]; }

  return lower_bound(dp.begin(), dp.end(), LONG_MAX) - dp.begin();
}

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  cout << lis({2, 3, 4, 1, 5}) << endl;
}
