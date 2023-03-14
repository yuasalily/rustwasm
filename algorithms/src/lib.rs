use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_test() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(57), false);
        assert_eq!(is_prime(30001), false);
        assert_eq!(is_prime(8635844967113809), false);
        assert_eq!(is_prime(9007199254740881), true);
    }
}
