#include <string>
#include <vector>

// rolling-hash.cpp --- Rolling-Hash implementation
// author: Seong Yong-ju <sei40kr@gmail.com>

const long long base = 9973;

struct rolling_hash64 {
  size_t n;
  std::vector<long long> hs, pw;

  rolling_hash64() {}

  explicit rolling_hash64(const std::string &s) {
    n = s.size();
    hs.assign(n + 1, 0);
    pw.assign(n + 1, 0);

    pw[0] = 1;
    for (size_t i = 0; i < n; ++i) {
      // Actually it prefers prime numbers to 2^64
      // This implementation may often cause conflictions!
      hs[i + 1] = hs[i] * base + s[i];
      pw[i + 1] = pw[i] * base;
    }
  }

  long long hash(int l, int r) { return hs[r] - hs[l] * pw[r - l]; }

  bool match(int l1, int r1, int l2, int r2) {
    return hash(l1, r1) == hash(l2, r2);
  }
};
