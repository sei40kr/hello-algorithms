// is-prime.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(dead_code)]
fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }

    for i in 2.. {
        if x < i * i {
            break;
        }
        if x % i == 0 {
            return false;
        }
    }

    return true;
}
