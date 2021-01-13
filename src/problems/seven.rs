use crate::math::is_prime;
use wasm_bindgen::prelude::*;

/**
* By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
*
* What is the 10 001st prime number?
*/

#[wasm_bindgen]
pub fn solve_7(n: usize) -> i64 {
    let mut i = 0;
    let mut count = 1;
    while i < n {
        count += 1;
        if is_prime(count) {
            i += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixth_prime() {
        assert_eq!(solve_7(6), 13);
    }

    #[test]
    fn ten_thousand_and_first_prime() {
        assert_eq!(solve_7(10001), 104743);
    }
}
