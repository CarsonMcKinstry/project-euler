use crate::math::gcd;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

pub(super) struct PythagoreanTriple(i64, i64, i64);

impl PythagoreanTriple {
    fn sum(&self) -> i64 {
        let Self(a, b, c) = *self;

        a + b + c
    }

    fn product(&self) -> i64 {
        let Self(a, b, c) = *self;

        a * b * c
    }
}

impl From<(i64, i64)> for PythagoreanTriple {
    fn from((m, n): (i64, i64)) -> Self {
        let a = m.pow(2) - n.pow(2);
        let b = 2 * m * n;
        let c = m.pow(2) + n.pow(2);

        Self(a, b, c)
    }
}

impl Mul<i64> for PythagoreanTriple {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        let Self(a, b, c) = self;

        Self(a * rhs, b * rhs, c * rhs)
    }
}

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
pub fn solve_9(s: i64) -> i64 {
    let mut p = PythagoreanTriple(0, 0, 0);

    let mut found: bool = false;

    let m_limit = (s as f64 / 2f64).sqrt();

    for m in (2..=(m_limit as i64)).step_by(2) {
        if (s / 2) % m == 0 {
            let mut k = if m % 2 == 0 {
                // m found
                m + 1
            } else {
                m + 2
            };

            while k < 2 * m && k <= s / (2 * m) {
                if s / (2 * m) % k == 0 && gcd(k, m) == 1 {
                    let d = s / 2 / (k * m);
                    let n = k - m;
                    p = PythagoreanTriple::from((m, n));

                    p = p * d;

                    found = true;
                    break;
                }
                k += 2;
            }
        }
        if found {
            break;
        }
    }

    p.product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triplet_for_12() {
        assert_eq!(solve_9(12), 60);
    }

    #[test]
    fn triplet_for_1000() {
        assert_eq!(solve_9(1000), 31875000);
    }
}
