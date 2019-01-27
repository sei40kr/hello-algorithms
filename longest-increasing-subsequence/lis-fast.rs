use std::env;
use std::i32;

// lis-fast.rs --- Longest Increasing Subsequence
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
// cf http://www.prefield.com/algorithm/dp/lis_fast.html
fn lis(vec: &Vec<i32>) -> usize {
    let mut dp = vec![i32::MAX; vec.len()];

    for &x in vec.into_iter() {
        // x extends largest subsequence, or will become end candidate of an
        // existing subsequence.
        let i = dp.binary_search(&x).unwrap();
        dp[i] = x;
    }

    return match dp.binary_search(&i32::MAX) {
        Ok(i) => i,
        Err(i) => i,
    };
}
