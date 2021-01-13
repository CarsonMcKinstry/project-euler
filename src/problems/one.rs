use wasm_bindgen::prelude::*;

/**
* If we list all the natural numbers below 10 that are multiples of 3 or 5,
* we get 3, 5, 6 and 9. The sum of these multiples is 23.
*
* Find the sum of all the multiples of 3 or 5 below 1000.
*/

#[wasm_bindgen]
pub fn solve_1(max: u32) -> u32 {
    (0..max).fold(0, |acc, val| {
        if val % 3 == 0 || val % 5 == 0 {
            acc + val
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_multiples_of_3_or_5_under_1000() {
        assert_eq!(solve_1(1000), 233168)
    }
}
