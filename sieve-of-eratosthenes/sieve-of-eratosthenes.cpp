#include <cmath>
#include <iostream>
#include <vector>

#define FOR(i, a, b) for (int i = (a); i < (b); i++)
#define EB emplace_back

#define FASTIO                      \
  ios_base::sync_with_stdio(false); \
  cin.tie(NULL);

// sieve-of-eratosthenes.cpp ---
// author: Seong Yong-ju <sei40kr@gmail.com>

using namespace std;

typedef vector<int> VI;

VI sieve(const int max) {
  bool isprime[max + 1];

  fill(isprime, isprime + 2, false);
  fill(isprime + 2, isprime + max + 1, true);

  int temp = floor(sqrt(max)) + 1;
  FOR(n, 2, temp) {
    if (!isprime[n]) {
      continue;
    }
    for (int m = n * 2; m <= max; m += n) {
      isprime[m] = false;
    }
  }

  VI primes;
  FOR(n, 2, max + 1) {
    if (isprime[n]) {
      primes.EB(n);
    }
  }

  return primes;
}

int main() {
  FASTIO;

  const VI primes = sieve(512);
  for (int i = 0, l = primes.size(); i < l; ++i) {
    printf("%5d", primes[i]);

    if (i % 13 == 12) {
      puts("");
    }
  }
}
