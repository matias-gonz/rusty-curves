use crate::felt::felt::Felt;

use super::ec_errors::ECError;

pub struct ECPoint {
    x: Felt,
    y: Felt,
    a: Felt,
    b: Felt,
}

impl ECPoint {
    pub fn new(x: Felt, y: Felt, a: Felt, b: Felt) -> Result<Self, ECError> {
        let point = ECPoint { x, y, a, b };
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
        let a = -Felt::new(1, 61);
        let b = Felt::new(0, 61);
        let x = Felt::new(4, 61);
        let y = Felt::new(4, 61);

        let point = ECPoint::new(x, y, a, b);
        assert!(point.is_err());
    }
}
