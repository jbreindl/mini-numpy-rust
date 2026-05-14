use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

pub mod errors;
pub mod vecs;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {

    use std::ops::{Add, Div, Mul, Rem, Sub};

    use crate::errors::VectorError;
    use crate::vecs;
    use pyo3::{
        exceptions::{PyTypeError, PyValueError},
        prelude::*,
    };
    use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
    use vecs::vector_ops::NumericVector;

    #[gen_stub_pyclass]
    #[pyclass(sequence)]
    /// Binding into Rust Vectors
    struct PyVector {
        data: VectorData,
    }

    enum VectorData {
        Int(NumericVector<i32>),
        Float(NumericVector<f32>),
    }

    #[gen_stub_pymethods]
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

        fn __add__(&self, other: &PyVector) -> PyResult<PyVector> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, add)?;
                    Ok(PyVector {
                        data: VectorData::Int(new_vec),
                    })
                }
                (VectorData::Float(v1), VectorData::Float(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, add)?;
                    Ok(PyVector {
                        data: VectorData::Float(new_vec),
                    })
                }
                _ => Err(PyTypeError::new_err("Types must match!")),
            }
        }

        fn __sub__(&self, other: &PyVector) -> PyResult<PyVector> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, sub)?;
                    Ok(PyVector {
                        data: VectorData::Int(new_vec),
                    })
                }
                (VectorData::Float(v1), VectorData::Float(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, sub)?;
                    Ok(PyVector {
                        data: VectorData::Float(new_vec),
                    })
                }
                _ => Err(PyTypeError::new_err("Types must match!")),
            }
        }
        fn __mul__(&self, other: &PyVector) -> PyResult<PyVector> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, mul)?;
                    Ok(PyVector {
                        data: VectorData::Int(new_vec),
                    })
                }
                (VectorData::Float(v1), VectorData::Float(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, sub)?;
                    Ok(PyVector {
                        data: VectorData::Float(new_vec),
                    })
                }
                _ => Err(PyTypeError::new_err("Types must match!")),
            }
        }
        fn __truediv__(&self, other: &PyVector) -> PyResult<PyVector> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, div)?;
                    Ok(PyVector {
                        data: VectorData::Int(new_vec),
                    })
                }
                (VectorData::Float(v1), VectorData::Float(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, div)?;
                    Ok(PyVector {
                        data: VectorData::Float(new_vec),
                    })
                }
                _ => Err(PyTypeError::new_err("Types must match!")),
            }
        }
        fn __mod__(&self, other: &PyVector) -> PyResult<PyVector> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, rem)?;
                    Ok(PyVector {
                        data: VectorData::Int(new_vec),
                    })
                }
                (VectorData::Float(v1), VectorData::Float(v2)) => {
                    let new_vec = v1.array_arithmetic(v2, rem)?;
                    Ok(PyVector {
                        data: VectorData::Float(new_vec),
                    })
                }
                _ => Err(PyTypeError::new_err("Types must match!")),
            }
        }

        fn __eq__(&self, other: &PyVector) -> PyResult<bool> {
            let data_pair = (&self.data, &other.data);
            match data_pair {
                (VectorData::Int(v1), VectorData::Int(v2)) => Ok(v1.is_equal(v2)),
                _ => Err(PyTypeError::new_err("Unsupported opperands")),
            }
        }
    }
    fn add<T: Add + Copy>(a: &T, b: &T) -> <T as Add>::Output {
        *a + *b
    }
    fn sub<T: Sub + Copy>(a: &T, b: &T) -> <T as Sub>::Output {
        *a - *b
    }
    fn div<T: Div + Copy>(a: &T, b: &T) -> <T as Div>::Output {
        *a / *b
    }

    fn mul<T: Mul + Copy>(a: &T, b: &T) -> <T as Mul>::Output {
        *a * *b
    }

    fn rem<T: Rem + Copy>(a: &T, b: &T) -> <T as Rem>::Output {
        *a % *b
    }
}
define_stub_info_gatherer!(stub_info);
