#include <bits/stdc++.h>

// lis.cpp
// author: Seong Yong-ju ( @sei40kr )

using namespace std;

long lis(const vector<int> &vec) {
  long res = 0;

  for (auto iter = vec.begin(); iter != vec.end(); ++iter) {
    res = max(res, count_if(vec.begin(), iter,
                            [iter](const int x) -> bool { return x < *iter; }));
  }

  return res;
}

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  cout << lis({2, 3, 4, 1, 5}) << endl;
}
