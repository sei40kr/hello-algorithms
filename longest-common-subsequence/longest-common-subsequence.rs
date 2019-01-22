use std::cmp::max;

// longest-common-subsequence.rs --- Longest Common Subsequence
// author: Seong Yong-ju <sei40kr@gmail.com>

macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn solve(s: String, t: String) -> usize {
    let n = s.len();
    let m = t.len();

    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = if s_chars[i] == t_chars[j] {
                dp[i][j] + 1
            } else {
                max(dp[i][j + 1], dp[i + 1][j])
            }
        }
    }

    return dp[n][m];
}

fn main() {
    let s = "abcd";
    let t = "becd";
    debug!(s);
    debug!(t);
    debug!(solve(String::from(s), String::from(t)));
}
