use wasm_bindgen::prelude::*;

/**
* A palindromic number reads the same both ways. The largest palindrome
* made from the product of two 2-digit numbers is 9009 = 91 × 99.

* Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn is_palindrome(mut n: i64) -> bool {
    let temp = n;
    let mut rev = 0;

    while n > 0 {
        let dig = n % 10;
        rev = rev * 10 + dig;
        n = n / 10;
    }

    temp == rev
}

#[wasm_bindgen]
pub fn solve_4(num_digits: u32) -> i64 {
    let start = 10i64.pow(num_digits - 1);
    let end = 10i64.pow(num_digits);

    let mut max: i64 = 0;

    for i in start..end {
        for j in start + 1..end {
            let temp = i * j;
            if is_palindrome(temp) && temp >= max {
                max = temp;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindromic_number_two_digits() {
        assert_eq!(solve_4(2), 9009);
    }

    #[test]
    fn largest_palindromic_number_three_digits() {
        assert_eq!(solve_4(3), 906609);
    }
}
