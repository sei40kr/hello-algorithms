// suffix-array.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
fn suffix_array(s: &str) -> Vec<&str> {
    let mut suffixes = (0..s.len()).map(|i| &s[i..]).collect::<Vec<_>>();
    suffixes.sort();
    suffixes
}
