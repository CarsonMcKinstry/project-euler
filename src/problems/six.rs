use wasm_bindgen::prelude::*;

/**
* The sum of the squares of the first ten natural numbers is,
*
* 1^2 + 2^2 + ... + 10^2 = 385
*
* The square of the sum of the first ten natural numbers is,
*
* (1 + 2 + ... + 10)^2 = 55^2 = 3025
*
* Hence the difference between the sum of the squares of the first
* ten natural numbers and the square of the sum is
*
* 3025 - 385 = 2640
*
* Find the difference between the sum of the squares of the
* first one hundred natural numbers and the square of the sum.
*/

#[wasm_bindgen]
pub fn solve_6(n_numbers: i64) -> i64 {
    let sum_of_squares: i64 = (0..=n_numbers).map(|n| n * n).sum();
    let square_of_sum: i64 = ((0..=n_numbers).sum::<i64>()).pow(2);

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_sum_of_squares_square_of_sums_10() {
        assert_eq!(solve_6(10), 2640);
    }

    #[test]
    fn diff_sum_of_squares_square_of_sums_100() {
        assert_eq!(solve_6(100), 25164150);
    }
}
