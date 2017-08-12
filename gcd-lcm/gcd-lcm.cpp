#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

using namespace std;

template<typename T>
T gcd(T n, T m) {
  if (n % m == 0) {
    return m;
  }

  return gcd(m, n % m);
}

template<typename T>
T lcm(T n, T m) {
  return n / gcd(n, m) * m;
}

int main() {
  cin.tie(0);

  cout << "Print 2 numbers [space-separated]: ";
  int a, b;
  cin >> a >> b;

  printf("gcd(%d, %d): %d\n", a, b, gcd(a, b));
  printf("lcm(%d, %d): %d", a, b, lcm(a, b));
}

