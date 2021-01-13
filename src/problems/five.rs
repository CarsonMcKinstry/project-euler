use crate::math::lcm;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/**
 * How to find LCM by Prime Factorization
Find all the prime factors of each given number.
List all the prime numbers found, as many times as they occur most often for any one given number.
Multiply the list of prime factors together to find the LCM.
The LCM(a,b) is calculated by finding the prime factorization of both a and b. Use the same process for the LCM of more than 2 numbers.

For example, for LCM(12,30) we find:

Prime factorization of 12 = 2 × 2 × 3
Prime factorization of 30 = 2 × 3 × 5
Using all prime numbers found as often as each occurs most often we take 2 × 2 × 3 × 5 = 60
Therefore LCM(12,30) = 60.
For example, for LCM(24,300) we find:

Prime factorization of 24 = 2 × 2 × 2 × 3
Prime factorization of 300 = 2 × 2 × 3 × 5 × 5
Using all prime numbers found as often as each occurs most often we take 2 × 2 × 2 × 3 × 5 × 5 = 600
Therefore LCM(24,300) = 600.
 */

/**
* 2520 is the smallest number that can be divided
* by each of the numbers from 1 to 10 without any remainder.
*
* What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

// This is SUPER DUPER slow
#[wasm_bindgen]
pub fn solve_5(min: i64, max: i64) -> i64 {
    (min..=max).fold(1, |acc, val| lcm(acc, val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_divisible_1_to_10() {
        assert_eq!(solve_5(1, 10), 2520);
    }

    #[test]
    fn smallest_divisible_1_to_20() {
        assert_eq!(solve_5(1, 20), 232792560);
    }
}
