use pyo3::prelude::*;

pub mod vecs;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {
    use crate::vecs;
    use pyo3::{
        exceptions::{PyTypeError, PyValueError},
        prelude::*,
    };
    use vecs::vector_ops::NumericVector;

    #[pyclass(sequence)]
    struct PyVector {
        data: VectorData,
    }

    enum VectorData {
        Int(NumericVector<i32>),
        Float(NumericVector<f32>),
    }

    #[pymethods]
    impl PyVector {
        #[new]
        fn new(input: &Bound<'_, PyAny>) -> PyResult<Self> {
            if let Ok(input) = input.extract::<Vec<i32>>() {
                return Ok(Self {
                    data: VectorData::Int(NumericVector::new(input)),
                });
            }
            if let Ok(input) = input.extract::<Vec<f32>>() {
                return Ok(Self {
                    data: VectorData::Float(NumericVector::new(input)),
                });
            }
            Err(PyTypeError::new_err("Type unsupported"))
        }

        fn __repr__(&self) -> String {
            match &self.data {
                VectorData::Int(v) => v.to_string(),
                VectorData::Float(v) => v.to_string(),
            }
        }
    }
}
