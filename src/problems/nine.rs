use wasm_bindgen::prelude::*;

/**
* A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
*
* a^2 + b^2 = c^2
*
* For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
*
* There exists exactly one Pythagorean triplet for which a + b + c = 1000.
* Find the product abc.
*/

#[wasm_bindgen]
pub fn solve_9(sum: u32) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triplet_for_12() {
        assert_eq!(solve_9(12), 60)
    }
}
