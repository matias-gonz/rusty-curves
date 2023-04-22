use thiserror::Error;

#[derive(Debug, Error)]
pub enum ECError {
    #[error("Point ({0}, {1}) is not on the curve y^2 = x^3 + {2}x + {3}")]
    PointNotOnCurve(u64, u64, u64, u64),
}
