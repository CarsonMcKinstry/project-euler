use wasm_bindgen::prelude::*;

/**
* Each new term in the Fibonacci sequence is generated by adding the previous
* two terms. By starting with 1 and 2, the first 10 terms will be:
*
* 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
*
* By considering the terms in the Fibonacci sequence whose values
* do not exceed four million, find the sum of the even-valued terms.
*/

struct Fib(u32, u32);

impl Fib {
    pub fn new() -> Self {
        Self(1, 2)
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(x, y) = *self;

        self.0 = y;
        self.1 = x + y;

        Some(self.1)
    }
}

#[wasm_bindgen]
pub fn solve_2(max: u32) -> u32 {
    let mut fib = Fib::new();
    let mut sum: u32 = 1 + fib.0;

    while let Some(n) = fib.next() {
        if n >= max {
            break;
        } else if n % 2 == 0 {
            sum += n;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_even_valued_fibonacci_numbers_under_4_million() {
        assert_eq!(solve_2(4000000), 4613732);
    }
}
