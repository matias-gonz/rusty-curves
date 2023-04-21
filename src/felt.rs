use std::ops::{Add, Sub};

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
    fn test_felt_display() {
        let f = Felt::new(5, 7);
        assert_eq!(format!("{}", f), "5 (mod 7)");
    }
}
