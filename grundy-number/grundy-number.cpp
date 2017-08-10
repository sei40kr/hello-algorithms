#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )
#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);

  int n;
  cout << "Input the number of coin groups [1-100]: ";
  cin >> n;

  int xs[n];
  cout << "Input the numbers of coins of each group [space-separated]: ";
  REP (i, n) {
    cin >> xs[i];
  }

  cout << endl;

  int grundy = accumulate(xs, xs + n, 0, [](int a, int b) -> int {
    return a ^ b;
  });
  cout << "grundy = " << grundy << endl;
  cout << "judge = " << (grundy == 0 ? "losing" : "winning") << endl;
}

