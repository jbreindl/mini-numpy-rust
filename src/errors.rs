use std::fmt;

use pyo3::{PyErr, exceptions::PyValueError};

/// Error if vector lengths don't match
#[derive(Debug, Clone)]
pub enum VectorError {
    MismatchedLengthError(usize, usize),
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedLengthError(a_len, b_len) => write!(
                f,
                "Lenghts must match! found lenghts {} and {}",
                a_len, b_len
            ),
        }
    }
}
impl From<VectorError> for PyErr {
    fn from(e: VectorError) -> PyErr {
        match e {
            VectorError::MismatchedLengthError(_a, _b) => PyValueError::new_err(e.to_string()),
        }
    }
}
