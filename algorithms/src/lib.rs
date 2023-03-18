use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_prime(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += 1;
    }
    0
}

pub fn prime_factorization(mut n: u64) -> HashMap<u64, u32> {
    let mut res = HashMap::new();
    let mut i: u64 = 2;
    while i * i <= n {
        while n % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        *res.entry(n).or_insert(0) += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_test() {
        assert_eq!(is_prime(1), 1);
        assert_eq!(is_prime(2), 0);
        assert_eq!(is_prime(3), 0);
        assert_eq!(is_prime(57), 3);
        assert_eq!(is_prime(30001), 19);
        assert_eq!(is_prime(8635844967113809), 89652331);
        assert_eq!(is_prime(9007199254740881), 0);
    }

    #[test]
    fn prime_factorization_test() {
        assert_eq!(prime_factorization(10), HashMap::from([(2, 1), (5, 1)]));
        assert_eq!(
            prime_factorization(120),
            HashMap::from([(2, 3), (3, 1), (5, 1)])
        );
        assert_eq!(
            prime_factorization(9007199254740881),
            HashMap::from([(9007199254740881, 1)])
        );
    }
}
