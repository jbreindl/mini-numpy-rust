use std::fmt;

use pyo3::{PyErr, exceptions::PyValueError};

/// Error if vector lengths don't match
#[derive(Debug, Clone)]
pub enum VectorError {
    IncompatibleShapeError(Vec<usize>, Vec<usize>),
    ShapeNotPossibleError(Vec<usize>, usize),
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleShapeError(a_shape, b_shape) => write!(
                f,
                "Shapes Incompatible! found lenghts {:#?} and {:#?}",
                a_shape, b_shape
            ),
            Self::ShapeNotPossibleError(shape, data_len) => write!(
                f,
                "Shape {:#?} not possible for data with length {data_len}",
                shape
            ),
        }
    }
}

impl From<VectorError> for PyErr {
    fn from(e: VectorError) -> PyErr {
        let msg = e.to_string();
        match e {
            VectorError::IncompatibleShapeError(_, _)
            | VectorError::ShapeNotPossibleError(_, _) => PyValueError::new_err(msg),
        }
    }
}
