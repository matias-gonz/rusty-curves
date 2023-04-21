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
    fn test_felt_display() {
        let f = Felt::new(5, 7);
        assert_eq!(format!("{}", f), "5 (mod 7)");
    }
}
