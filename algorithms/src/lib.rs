use std::collections::HashMap;

use wasm_bindgen::prelude::*;

/// 素数判定をする関数です。
/// 返り値が0なら素数、0以外なら少なくともその値で割り切れる。
/// * `n` - 1以上の18446744073709551615以下の自然数
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

/// 素因数分解をする関数です。
/// * `n` - 1以上の18446744073709551615以下の自然数
#[wasm_bindgen]
pub fn prime_factorization(mut n: u64) -> String {
    let mut res: HashMap<u64, u32> = HashMap::new();
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
    let mut res_vec: Vec<(&u64, &u32)> = res.iter().collect();
    res_vec.sort();
    let mut res_string = String::new();
    for (&k, &v) in res_vec {
        res_string = if res_string.is_empty() {
            format!("{}{}-{}", res_string, k, v)
        } else {
            format!("{}-{}-{}", res_string, k, v)
        }
    }
    res_string
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
        assert_eq!(prime_factorization(10), String::from("2-1-5-1"));
        assert_eq!(prime_factorization(360), String::from("2-3-3-2-5-1"));
        assert_eq!(
            prime_factorization(9007199254740881),
            String::from("9007199254740881-1")
        );
    }
}
