use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Mul, Neg},
};

use crate::felt::felt::Felt;

use super::ec_errors::ECError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn infinity(a: Felt, b: Felt) -> ECPoint {
        ECPoint {
            x: Felt::new(0, a.modulus()),
            y: Felt::new(0, a.modulus()),
            a,
            b,
            infinity: true,
        }
    }

    pub fn order(&self) -> u64 {
        let mut gi = *self;
        let mut order = 1;
        let infinity = ECPoint::infinity(self.a, self.b);
        while gi != infinity {
            order += 1;
            gi += *self;
        }
        order
    }

    // x*self = target
    pub fn solve_dlp_brute_force(&self, target: ECPoint) -> Option<u64> {
        let mut xp = *self;
        let mut x = 1;
        let infinity = ECPoint::infinity(self.a, self.b);
        while xp != infinity {
            if xp == target {
                return Some(x);
            }
            x += 1;
            xp += *self;
        }
        None
    }

    // x*self = target
    pub fn solve_dlp_baby_step_giant_step(&self, target: ECPoint) -> Option<u64> {
        let m = (self.order() as f64).sqrt().ceil() as u64;
        let mut baby_steps = HashMap::new();
        let mut pi = *self;
        for i in 1..m {
            baby_steps.insert(pi, i);
            pi += *self;
        }

        let mp = m * *self;
        let mut jmp = ECPoint::infinity(self.a, self.b); // j*m*p not jump :P
        for j in 0..m {
            let q = target + (-jmp);

            if let Some(i) = baby_steps.get(&q) {
                return Some(m * j + i);
            }

            jmp += mp;
        }

        None
    }

    // Naive implementation of getting all points on the curve
    #[allow(dead_code)]
    fn get_all_points(a: Felt, b: Felt) -> HashSet<ECPoint> {
        let mut points = HashSet::new();
        points.insert(ECPoint::infinity(a, b));

        for x in 0..a.modulus() {
            for y in 0..a.modulus() {
                let felt_x = Felt::new(x, a.modulus());
                let felt_y = Felt::new(y, a.modulus());

                let _ = match ECPoint::new(felt_x, felt_y, a, b) {
                    Ok(point) => points.insert(point),
                    Err(_) => true,
                };
            }
        }

        points
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

        let s = if self == other {
            let felt_3 = Felt::new(3, self.a.modulus());
            let felt_2 = Felt::new(2, self.a.modulus());

            (felt_3 * self.x.pow(2) + self.a) / (felt_2 * self.y)
        } else {
            (other.y - self.y) / (other.x - self.x)
        };

        let x = s.pow(2) - self.x - other.x;
        let y = s * (self.x - x) - self.y;

        ECPoint::new(x, y, self.a, self.b).unwrap()
    }
}

impl AddAssign for ECPoint {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for ECPoint {
    type Output = Self;

    fn neg(self) -> Self {
        if self.infinity {
            return self;
        }
        ECPoint::new(self.x, -self.y, self.a, self.b).unwrap()
    }
}

impl Mul<u64> for ECPoint {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        let mut result = ECPoint::infinity(self.a, self.b);
        let mut current = self;
        let mut i = other;

        while i > 0 {
            if i % 2 == 1 {
                result += current;
            }
            i >>= 1;
            current += current;
        }

        result
    }
}

impl Mul<ECPoint> for u64 {
    type Output = ECPoint;

    fn mul(self, other: ECPoint) -> ECPoint {
        other * self
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
    fn test_add_point_with_itself() {
        let modulus = 101;
        let a = Felt::new(5, modulus);
        let b = Felt::new(13, modulus);
        let x = Felt::new(24, modulus);
        let y = Felt::new(25, modulus);

        let p1 = ECPoint::new(x, y, a, b).unwrap();

        let p3 = p1 + p1;
        assert_eq!(
            p3,
            ECPoint::new(Felt::new(67, modulus), Felt::new(38, modulus), a, b).unwrap()
        );
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

    #[test]
    fn test_multiply_by_one_should_equal_original() {
        let modulus = 37;
        let a = Felt::new(7, modulus);
        let b = Felt::new(13, modulus);
        let x = Felt::new(5, modulus);
        let y = Felt::new(5, modulus);

        let p = ECPoint::new(x, y, a, b).unwrap();
        let p2 = p * 1;
        assert_eq!(p2, p);
    }

    #[test]
    fn test_multiply_by_two() {
        let modulus = 37;
        let a = Felt::new(7, modulus);
        let b = Felt::new(13, modulus);
        let x = Felt::new(5, modulus);
        let y = Felt::new(5, modulus);

        let p = ECPoint::new(x, y, a, b).unwrap();
        let p2 = p * 2;
        assert_eq!(
            p2,
            ECPoint::new(Felt::new(1, modulus), Felt::new(13, modulus), a, b).unwrap()
        );
    }

    #[test]
    fn test_multiply_by_three() {
        let modulus = 37;
        let a = Felt::new(7, modulus);
        let b = Felt::new(13, modulus);
        let x = Felt::new(5, modulus);
        let y = Felt::new(5, modulus);

        let p = ECPoint::new(x, y, a, b).unwrap();
        let p2 = p * 3;
        assert_eq!(
            p2,
            ECPoint::new(Felt::new(35, modulus), Felt::new(18, modulus), a, b).unwrap()
        );
    }

    #[test]
    fn test_multiply_by_ten() {
        let modulus = 37;
        let a = Felt::new(7, modulus);
        let b = Felt::new(13, modulus);
        let x = Felt::new(5, modulus);
        let y = Felt::new(5, modulus);

        let p = ECPoint::new(x, y, a, b).unwrap();
        let p2 = p * 10;
        assert_eq!(
            p2,
            ECPoint::new(Felt::new(22, modulus), Felt::new(14, modulus), a, b).unwrap()
        );
    }

    #[test]
    fn test_compare_multiplication_and_addition() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x = Felt::new(18, modulus);
        let y = Felt::new(26, modulus);
        let p = ECPoint::new(x, y, a, b).unwrap();

        let mut p_add = ECPoint::infinity(a, b);
        for i in 1..1000 {
            p_add += p;
            let p_mul = i * p;
            assert_eq!(p_add, p_mul);
        }
    }

    #[test]
    fn test_multiply() {
        let modulus = 1021;
        let a = -Felt::new(3, modulus);
        let b = -Felt::new(3, modulus);
        let x = Felt::new(379, modulus);
        let y = Felt::new(1011, modulus);
        let p = ECPoint::new(x, y, a, b).unwrap();
        let k = 655;
        let kp = ECPoint::new(Felt::new(388, modulus), Felt::new(60, modulus), a, b).unwrap();
        assert_eq!(k * p, kp);
    }

    #[test]
    fn test_get_all_points_simple() {
        let modulus = 7;
        let a = Felt::new(2, modulus);
        let b = Felt::new(3, modulus);

        let points = ECPoint::get_all_points(a, b);
        assert_eq!(points.len(), 6);
    }

    #[test]
    fn test_get_all_points_big_curve() {
        let modulus = 1021;
        let a = -Felt::new(3, modulus);
        let b = -Felt::new(3, modulus);
        println!("a = {}", a);
        let points = ECPoint::get_all_points(a, b);
        assert_eq!(points.len(), 1039);
    }

    #[test]
    fn test_get_order() {
        let modulus = 1021;
        let a = -Felt::new(3, modulus);
        let b = -Felt::new(3, modulus);
        let x = Felt::new(379, modulus);
        let y = Felt::new(1011, modulus);
        let p = ECPoint::new(x, y, a, b).unwrap();
        let order = p.order();
        assert_eq!(order, 1039);
    }

    #[test]
    fn test_solve_dlp_brute_force() {
        let modulus = 1021;
        let a = Felt::new(905, modulus);
        let b = Felt::new(100, modulus);
        let x = Felt::new(1006, modulus);
        let y = Felt::new(416, modulus);
        let p = ECPoint::new(x, y, a, b).unwrap();

        let x = Felt::new(612, modulus);
        let y = Felt::new(827, modulus);
        let q = ECPoint::new(x, y, a, b).unwrap();

        let k = 687;
        let solved_k = p.solve_dlp_brute_force(q).unwrap();

        assert_eq!(solved_k, k);
        assert_eq!(solved_k * p, q);
    }

    #[test]
    fn test_solve_dlp_baby_step_giant_step() {
        let modulus = 1021;
        let a = Felt::new(905, modulus);
        let b = Felt::new(100, modulus);
        let x = Felt::new(1006, modulus);
        let y = Felt::new(416, modulus);
        let p = ECPoint::new(x, y, a, b).unwrap();

        let x = Felt::new(612, modulus);
        let y = Felt::new(827, modulus);
        let q = ECPoint::new(x, y, a, b).unwrap();

        let k = 687;
        let solved_k = p.solve_dlp_baby_step_giant_step(q).unwrap();

        assert_eq!(solved_k, k);
        assert_eq!(solved_k * p, q);
    }

    #[test]
    fn test_display() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);
        let x = Felt::new(18, modulus);
        let y = Felt::new(26, modulus);

        let p1 = ECPoint::new(x, y, a, b).unwrap();
        assert_eq!(format!("{}", p1), "(18, 26)");
    }

    #[test]
    fn test_display_infinity() {
        let modulus = 37;
        let a = Felt::new(3, modulus);
        let b = Felt::new(7, modulus);

        let p1 = ECPoint::infinity(a, b);
        assert_eq!(format!("{}", p1), "Infinity");
    }
}
