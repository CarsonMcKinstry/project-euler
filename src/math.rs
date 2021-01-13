pub(crate) fn prime_factors(n: i64) -> Vec<i64> {
    let mut s = n;
    let mut factors: Vec<i64> = Vec::new();

    while s % 2 == 0 {
        factors.push(2);
        s = s / 2;
    }

    for i in (3..(s as f64).sqrt() as i64).step_by(2) {
        while s % i == 0 {
            factors.push(i);
            s = s / i;
        }
    }

    if s > 2 {
        factors.push(s);
    }

    factors
}

pub(crate) fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

pub(crate) fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

/**
*     while i ** 2 <= n:
       if n % i == 0 or n % (i + 2) == 0:
           return False
       i += 6
   return True
*/
pub(crate) fn is_prime(n: i64) -> bool {
    if n <= 3 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i: i64 = 5;

        while i.pow(2) <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            } else {
                i += 6;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factors_of_12345() {
        assert_eq!(prime_factors(12345), vec![3, 5, 823]);
    }

    #[test]
    fn gcd_of_6_and_8() {
        assert_eq!(gcd(6, 8), 2);
    }

    #[test]
    fn gcd_is_communicative() {
        assert_eq!(gcd(6, 8), gcd(8, 6));
    }

    #[test]
    fn lcm_of_6_and_8() {
        assert_eq!(lcm(6, 8), 24);
    }

    #[test]
    fn lcm_is_communicative() {
        assert_eq!(lcm(6, 8), lcm(8, 6));
    }

    #[test]
    fn is_prime_2() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn is_prime_5() {
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn is_prime_4() {
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn is_prime_1877() {
        assert_eq!(is_prime(1877), true);
    }

    #[test]
    fn is_prime_1876() {
        assert_eq!(is_prime(1876), false);
    }
}
