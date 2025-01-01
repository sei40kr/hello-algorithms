pub fn z(s: &str) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut z = vec![0; n];
    z[0] = n;

    let mut left = 0;
    let mut right = 0;

    for i in 1..n {
        if right <= i {
            let mut j = 0;
            while i + j < n && chars[j] == chars[i + j] {
                j += 1;
            }
            z[i] = j;

            if j > 0 {
                left = i;
                right = i + j;
            }
        } else {
            let k = i - left;
            if z[k] < right - i {
                z[i] = z[k];
            } else {
                left = i;
                while right < n && chars[right] == chars[right - left] {
                    right += 1;
                }
                z[i] = right - left;
            }
        }
    }

    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z() {
        assert_eq!(z("aabaacd"), vec![7, 1, 0, 2, 1, 0, 0]);
    }

    #[test]
    fn test_z_when_repeating_pattern() {
        assert_eq!(z("aaaaa"), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_z_when_alternating_pattern() {
        assert_eq!(z("abababab"), vec![8, 0, 6, 0, 4, 0, 2, 0]);
    }

    #[test]
    fn test_z_when_no_matches_after_first_character() {
        assert_eq!(z("abcdef"), vec![6, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_z_when_single_character() {
        assert_eq!(z("a"), vec![1]);
    }
}
