use crate::math::prime_factors;
use wasm_bindgen::prelude::*;

/**
* The prime factors of 13195 are 5, 7, 13 and 29.
*
* What is the largest prime factor of the number 600851475143 ?
*/

#[wasm_bindgen]
pub fn solve_3(n: i64) -> i64 {
    let factors = prime_factors(n);

    *factors.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_factor_of_13195() {
        assert_eq!(solve_3(13195), 29)
    }

    #[test]
    fn largest_prime_factor_of_600851475143() {
        assert_eq!(solve_3(600851475143), 6857);
    }
}
