use crate::math::is_prime;
use wasm_bindgen::prelude::*;

/**
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

#[wasm_bindgen]
pub fn solve_10(max: i64) -> i64 {
    (2..max).fold(0, |mut acc, i| {
        if is_prime(i) {
            acc += i;
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_primes_below_10() {
        assert_eq!(solve_10(10), 17)
    }

    #[test]
    fn sum_of_primes_below_2m() {
        assert_eq!(solve_10(2000000), 142913828922);
    }
}
