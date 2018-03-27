#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define FOR(i, a, b) for (int i = (a); i < (b); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);

  int n;
  cout << "Calculate Nth fibonacci number. Input n [1-1000]: ";
  cin >> n;

  long memo[n] = {1, 1,};
  FOR(i, 2, n) {
    memo[i] = memo[i - 1] + memo[i - 2];
  }

  cout << memo[n - 1];
}

