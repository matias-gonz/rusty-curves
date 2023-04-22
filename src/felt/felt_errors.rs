use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeltError {
    #[error("{0} is not invertible (mod {1})")]
    NotInvertible(u64, u64),
}
