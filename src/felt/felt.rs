use std::ops::{Add, Div, Mul, Neg, Sub};

use super::felt_errors::FeltError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Felt {
    value: u64,
    modulus: u64,
}

impl Felt {
    pub fn new(value: u64, modulus: u64) -> Self {
        Felt {
            value: value % modulus,
            modulus,
        }
    }

    // Extended Euclidean algorithm
    pub fn inverse(&self) -> Result<Self, FeltError> {
        let mut t = 0_i64;
        let mut new_t = 1;
        let mut r = self.modulus as i64;
        let mut new_r = self.value as i64;

        while new_r != 0 {
            let quotient = r / new_r;

            let old_t = t;
            t = new_t;
            new_t = old_t - quotient * new_t;

            let old_r = r;
            r = new_r;
            new_r = old_r - quotient * new_r;
        }

        if r > 1 {
            return Err(FeltError::NotInvertible(self.value, self.modulus));
        }

        if t < 0 {
            t += self.modulus as i64;
        }

        Ok(Felt::new(t as u64, self.modulus))
    }

    pub fn pow(&self, exponent: u64) -> Self {
        let mut result = Felt::new(1, self.modulus);
        let mut base = *self;
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base;
            }
            exp >>= 1;
            base = base * base;
        }

        result
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Add for Felt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot add two Felt values with different moduli");
        }
        Felt::new(self.value + other.value, self.modulus)
    }
}

impl Sub for Felt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot subtract two Felt values with different moduli");
        }
        if self.value < other.value {
            return Felt::new(self.value + self.modulus - other.value, self.modulus);
        }
        Felt::new(self.value - other.value, self.modulus)
    }
}

impl Mul for Felt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot multiply two Felt values with different moduli");
        }
        Felt::new(self.value * other.value, self.modulus)
    }
}

impl Div for Felt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot divide two Felt values with different moduli");
        }
        if other.value == 0 {
            panic!("Cannot divide by zero");
        }
        match other.inverse() {
            Ok(inverse) => self * inverse,
            Err(e) => panic!("{}", e),
        }
    }
}

impl Neg for Felt {
    type Output = Self;

    fn neg(self) -> Self {
        Felt::new(self.modulus - self.value, self.modulus)
    }
}

impl std::fmt::Display for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulus)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_felt_new() {
        let f = Felt::new(5, 7);
        assert_eq!(f.value, 5);
        assert_eq!(f.modulus, 7);
    }

    #[test]
    fn test_felt_new_value_equals_modulus_should_have_value_zero() {
        let f = Felt::new(9, 9);
        assert_eq!(f.value, 0);
        assert_eq!(f.modulus, 9);
    }

    #[test]
    fn test_add_with_no_overflow() {
        let f1 = Felt::new(1, 7);
        let f2 = Felt::new(3, 7);
        let f3 = f1 + f2;
        assert_eq!(f3.value, 4);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_add_with_overflow() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 7);
        let f3 = f1 + f2;
        assert_eq!(f3.value, 1);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot add two Felt values with different moduli")]
    fn test_add_with_different_modulus_should_panic() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 9);
        let _ = f1 + f2;
    }

    #[test]
    fn test_subtract_with_no_overflow() {
        let f1 = Felt::new(3, 7);
        let f2 = Felt::new(1, 7);
        let f3 = f1 - f2;
        assert_eq!(f3.value, 2);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_subtract_with_overflow() {
        let f1 = Felt::new(2, 7);
        let f2 = Felt::new(5, 7);
        let f3 = f1 - f2;
        assert_eq!(f3.value, 4);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract two Felt values with different moduli")]
    fn test_subtract_with_different_modulus_should_panic() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 9);
        let _ = f1 - f2;
    }

    #[test]
    fn test_subtract_self_should_equal_zero() {
        let f1 = Felt::new(5, 7);
        let f2 = f1 - f1;
        assert_eq!(f2.value, 0);
        assert_eq!(f2.modulus, 7);
    }

    #[test]
    fn test_multiply_with_no_overflow() {
        let f1 = Felt::new(3, 7);
        let f2 = Felt::new(2, 7);
        let f3 = f1 * f2;
        assert_eq!(f3.value, 6);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_multiply_with_overflow() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 7);
        let f3 = f1 * f2;
        assert_eq!(f3.value, 1);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_multiply_with_zero() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(0, 7);
        let f3 = f1 * f2;
        assert_eq!(f3.value, 0);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply two Felt values with different moduli")]
    fn test_multiply_with_different_modulus_should_panic() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 9);
        let _ = f1 * f2;
    }

    #[test]
    fn test_inverse_of_one_should_be_one() {
        let f = Felt::new(1, 7);
        let f_inv = f.inverse().unwrap();
        assert_eq!(f_inv.value, 1);
        assert_eq!(f_inv.modulus, 7);
    }

    #[test]
    fn test_inverse_of_three_modulus_seven_should_be_five() {
        let f = Felt::new(3, 7);
        let f_inv = f.inverse().unwrap();
        assert_eq!(f_inv.value, 5);
        assert_eq!(f_inv.modulus, 7);
    }

    #[test]
    fn test_multiply_with_inverse_should_equal_one() {
        let f = Felt::new(3, 7);
        let f_inv = f.inverse().unwrap();
        let f_one = f * f_inv;
        assert_eq!(f_one.value, 1);
        assert_eq!(f_one.modulus, 7);
    }

    #[test]
    fn test_divide_with_no_overflow() {
        let f1 = Felt::new(6, 7);
        let f2 = Felt::new(2, 7);
        let f3 = f1 / f2;
        assert_eq!(f3.value, 3);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_divide_with_overflow() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 7);
        let f3 = f1 / f2;
        assert_eq!(f3.value, 4);
        assert_eq!(f3.modulus, 7);
    }

    #[test]
    fn test_divide_and_multiply_should_equal_original() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 7);
        let f3 = f1 / f2;
        let f4 = f3 * f2;
        assert_eq!(f4.value, 5);
        assert_eq!(f4.modulus, 7);
    }

    #[test]
    #[should_panic(expected = "Cannot divide two Felt values with different moduli")]
    fn test_divide_with_different_modulus_should_panic() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 9);
        let _ = f1 / f2;
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_with_zero_should_panic() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(0, 7);
        let _ = f1 / f2;
    }

    #[test]
    fn test_felt_pow_with_exponent_one() {
        let f = Felt::new(5, 7);
        let f_pow = f.pow(1);
        assert_eq!(f_pow.value, 5);
        assert_eq!(f_pow.modulus, 7);
    }

    #[test]
    fn test_felt_pow_with_exponent_two() {
        let f = Felt::new(5, 7);
        let f_pow = f.pow(2);
        assert_eq!(f_pow.value, 4);
        assert_eq!(f_pow.modulus, 7);
    }

    #[test]
    fn test_felt_pow_with_exponent_power_of_two() {
        let f = Felt::new(5, 97);
        let f_pow = f.pow(32);
        assert_eq!(f_pow.value, 35);
        assert_eq!(f_pow.modulus, 97);
    }

    #[test]
    fn test_felt_pow_with_exponent_not_a_power_of_two() {
        let f = Felt::new(12, 101);
        let f_pow = f.pow(52);
        assert_eq!(f_pow.value, 58);
        assert_eq!(f_pow.modulus, 101);
    }

    #[test]
    fn test_felt_pow_with_exponent_one_should_equal_original() {
        let f = Felt::new(5, 7);
        let f_pow = f.pow(1);
        assert_eq!(f_pow.value, 5);
        assert_eq!(f_pow.modulus, 7);
    }

    #[test]
    fn test_felt_pow_with_exponent_zero_should_equal_one() {
        let f = Felt::new(5, 7);
        let f_pow = f.pow(0);
        assert_eq!(f_pow.value, 1);
        assert_eq!(f_pow.modulus, 7);
    }

    #[test]
    fn test_negative_felt() {
        let f = Felt::new(5, 7);
        let f_neg = -f;
        assert_eq!(f_neg.value, 2);
        assert_eq!(f_neg.modulus, 7);
    }

    #[test]
    fn test_felt_equal() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(5, 7);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_felt_not_equal() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(3, 7);
        assert_ne!(f1, f2);
    }

    #[test]
    fn test_felt_not_equal_with_different_modulus() {
        let f1 = Felt::new(5, 7);
        let f2 = Felt::new(5, 9);
        assert_ne!(f1, f2);
    }

    #[test]
    fn test_felt_display() {
        let f = Felt::new(5, 7);
        assert_eq!(format!("{}", f), "5 (mod 7)");
    }
}
