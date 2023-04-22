use std::{
    fmt::{Display, Formatter},
    ops::{Add, Neg},
};

use crate::felt::felt::Felt;

use super::ec_errors::ECError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ECPoint {
    x: Felt,
    y: Felt,
    a: Felt,
    b: Felt,
    infinity: bool,
}

impl ECPoint {
    pub fn new(x: Felt, y: Felt, a: Felt, b: Felt) -> Result<Self, ECError> {
        let point = ECPoint {
            x,
            y,
            a,
            b,
            infinity: false,
        };
        point.verify_point()?;
        Ok(point)
    }

    fn verify_point(&self) -> Result<(), ECError> {
        let lhs = self.y.pow(2);
        let rhs = self.x.pow(3) + self.a * self.x + self.b;

        if lhs == rhs {
            Ok(())
        } else {
            Err(ECError::PointNotOnCurve(
                self.x.value(),
                self.y.value(),
                self.a.value(),
                self.b.value(),
            ))
        }
    }

    fn infinity(a: Felt, b: Felt) -> ECPoint {
        ECPoint {
            x: Felt::new(0, a.modulus()),
            y: Felt::new(0, a.modulus()),
            a,
            b,
            infinity: true,
        }
    }
}

impl Add for ECPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {}, {} are not on the same curve", self, other);
        }

        // P + 0 = P
        if self.infinity {
            return other;
        }
        if other.infinity {
            return self;
        }

        // P + (-P) = 0
        if self == -other {
            return ECPoint::infinity(self.a, self.b);
        }

        let s = (other.y - self.y) / (other.x - self.x);
        let x = s.pow(2) - self.x - other.x;
        let y = s * (self.x - x) - self.y;

        ECPoint::new(x, y, self.a, self.b).unwrap()
    }
}

impl Neg for ECPoint {
    type Output = Self;

    fn neg(self) -> Self {
        ECPoint::new(self.x, -self.y, self.a, self.b).unwrap()
    }
}

impl Display for ECPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.infinity {
            write!(f, "Infinity")
        } else {
            write!(f, "({}, {})", self.x.value(), self.y.value())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::felt::felt::Felt;

    #[test]
    fn test_point_on_curve() {
        let a = -Felt::new(1, 61);
        let b = Felt::new(0, 61);
        let x = Felt::new(8, 61);
        let y = Felt::new(4, 61);

        let point = ECPoint::new(x, y, a, b);
        assert!(point.is_ok());
    }

    #[test]
    fn test_another_point_on_curve() {
        let a = -Felt::new(1, 61);
        let b = Felt::new(0, 61);
        let x = Felt::new(24, 61);
        let y = Felt::new(40, 61);

        let point = ECPoint::new(x, y, a, b);
        assert!(point.is_ok());
    }

    #[test]
    fn test_point_not_on_curve() {
        let modulus = 61;
        let a = -Felt::new(1, modulus);
        let b = Felt::new(0, modulus);
        let x = Felt::new(4, modulus);
        let y = Felt::new(4, modulus);

        let point = ECPoint::new(x, y, a, b);
        assert!(point.is_err());
    }

    #[test]
    fn test_add_two_points() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x1 = Felt::new(18, modulus);
        let y1 = Felt::new(26, modulus);
        let x2 = Felt::new(24, modulus);
        let y2 = Felt::new(19, modulus);

        let p1 = ECPoint::new(x1, y1, a, b).unwrap();
        let p2 = ECPoint::new(x2, y2, a, b).unwrap();

        let p3 = p1 + p2;
        assert_eq!(
            p3,
            ECPoint::new(Felt::new(20, modulus), Felt::new(1, modulus), a, b).unwrap()
        );
    }

    #[test]
    fn test_add_point_with_its_addition_inverse() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x = Felt::new(18, modulus);
        let y = Felt::new(26, modulus);

        let p1 = ECPoint::new(x, y, a, b).unwrap();
        let p2 = ECPoint::new(x, -y, a, b).unwrap();

        let p3 = p1 + p2;
        assert_eq!(p3, ECPoint::infinity(a, b));
    }

    #[test]
    #[should_panic(expected = "Points (18, 26), (8, 4) are not on the same curve")]
    fn test_add_points_from_different_curves_should_panic() {
        let modulus1 = 37;
        let a1 = Felt::new(3, modulus1);
        let b1 = Felt::new(7, modulus1);
        let x1 = Felt::new(18, modulus1);
        let y1 = Felt::new(26, modulus1);

        let modulus2 = 61;
        let a2 = -Felt::new(1, modulus2);
        let b2 = Felt::new(0, modulus2);
        let x2 = Felt::new(8, modulus2);
        let y2 = Felt::new(4, modulus2);

        let p1 = ECPoint::new(x1, y1, a1, b1).unwrap();
        let p2 = ECPoint::new(x2, y2, a2, b2).unwrap();

        let _ = p1 + p2;
    }

    #[test]
    fn test_add_point_with_infinity() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x = Felt::new(18, modulus);
        let y = Felt::new(26, modulus);

        let p1 = ECPoint::new(x, y, a, b).unwrap();
        let p2 = ECPoint::infinity(a, b);

        let p3 = p1 + p2;
        assert_eq!(p3, p1);
    }

    #[test]
    fn test_add_infinity_with_point() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x = Felt::new(18, modulus);
        let y = Felt::new(26, modulus);

        let p1 = ECPoint::new(x, y, a, b).unwrap();
        let p2 = ECPoint::infinity(a, b);

        let p3 = p2 + p1;
        assert_eq!(p3, p1);
    }

    #[test]
    fn test_add_infinity_with_infinity() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);

        let p1 = ECPoint::infinity(a, b);
        let p2 = ECPoint::infinity(a, b);

        let p3 = p1 + p2;
        assert_eq!(p3, ECPoint::infinity(a, b));
    }
}