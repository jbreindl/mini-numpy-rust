use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

pub mod errors;
pub mod vecs;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {

    use crate::vecs;
    use pyo3::{exceptions::PyTypeError, prelude::*};
    use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyfunction, gen_stub_pymethods};
    use vecs::vector_ops::Tensor;

    #[gen_stub_pyclass]
    #[pyclass(sequence)]
    /// Binding into Rust Vectors
    struct PyVector {
        data: VectorData,
    }

    enum VectorData {
        Int(Tensor<i32>),
        Float(Tensor<f32>),
    }
    impl From<Tensor<i32>> for PyVector {
        fn from(t: Tensor<i32>) -> Self {
            PyVector {
                data: VectorData::Int(t),
            }
        }
    }

    impl From<Tensor<f32>> for PyVector {
        fn from(t: Tensor<f32>) -> Self {
            PyVector {
                data: VectorData::Float(t),
            }
        }
    }

    macro_rules! binary_op {
    ($tensor_1: expr, $tensor_2:expr, $op:tt) => {
        match (&$tensor_1.data, &$tensor_2.data) {
            (VectorData::Int(v1), VectorData::Int(v2)) => Ok(((v1 $op v2)?).into()),
            (VectorData::Float(v1), VectorData::Float(v2)) => Ok((v1 $op v2)?.into()),
            _ => Err(PyTypeError::new_err("Types must match!")),
        }
    };
}

    #[gen_stub_pymethods]
    #[pymethods]
    impl PyVector {
        #[new]
        #[pyo3(signature=(input, shape=None))]
        fn new(input: &Bound<'_, PyAny>, shape: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
            let shape: Option<Vec<usize>> = match shape {
                Some(shape) => {
                    if let Ok(shape) = shape.extract::<Vec<usize>>() {
                        Some(shape)
                    } else {
                        return Err(PyTypeError::new_err("Shape is not of correct type"));
                    }
                }
                None => None,
            };
            if let Ok(input) = input.extract::<Vec<i32>>() {
                Ok(Tensor::new(input, shape)?.into())
            } else if let Ok(input) = input.extract::<Vec<f32>>() {
                Ok(Tensor::new(input, shape)?.into())
            } else {
                Err(PyTypeError::new_err("Type unsupported"))
            }
        }

        fn __repr__(&self) -> String {
            match &self.data {
                VectorData::Int(v) => v.to_string(),
                VectorData::Float(v) => v.to_string(),
            }
        }

        fn __add__(&self, other: &PyVector) -> PyResult<PyVector> {
            binary_op!(self, &other, +)
        }

        fn __sub__(&self, other: &PyVector) -> PyResult<PyVector> {
            binary_op!(self, &other, -)
        }
        fn __mul__(&self, other: &PyVector) -> PyResult<PyVector> {
            binary_op!(self, &other, *)
        }
        fn __truediv__(&self, other: &PyVector) -> PyResult<PyVector> {
            binary_op!(self, &other, /)
        }
        fn __mod__(&self, other: &PyVector) -> PyResult<PyVector> {
            binary_op!(self, &other, %)
        }

        fn __eq__(&self, other: &PyVector) -> PyResult<bool> {
            match (&self.data, &other.data) {
                (VectorData::Int(v1), VectorData::Int(v2)) => Ok(v1.is_equal(v2)),
                _ => Err(PyTypeError::new_err("Unsupported opperands")),
            }
        }
        fn __getitem__<'py>(
            &self,
            py: Python<'py>,
            idx: Bound<'py, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            if let Ok(idx) = idx.extract::<usize>() {
                match &self.data {
                    VectorData::Int(vec) => Ok(vec[idx].into_pyobject(py)?.into_any().clone()),
                    VectorData::Float(vec) => Ok(vec[idx].into_pyobject(py)?.into_any().clone()),
                }
            } else {
                Err(PyTypeError::new_err("Supplied type not supported!"))
            }
        }
        fn __setitem__<'py>(
            &mut self,
            _py: Python<'py>,
            idx: Bound<'py, PyAny>,
            value: Bound<'py, PyAny>,
        ) -> Result<(), PyErr> {
            if let Ok(idx) = idx.extract::<usize>() {
                match &mut self.data {
                    VectorData::Int(vec) => {
                        if let Ok(value) = value.extract::<i32>() {
                            vec[idx] = value;
                            Ok(())
                        } else {
                            Err(PyTypeError::new_err("Vector Type Mismatch"))
                        }
                    }
                    VectorData::Float(vec) => {
                        if let Ok(value) = value.extract::<f32>() {
                            vec[idx] = value;
                            Ok(())
                        } else {
                            Err(PyTypeError::new_err("Vector Type Mismatch"))
                        }
                    }
                }
            } else {
                Err(PyTypeError::new_err("Supplied index not supported!"))
            }
        }
    }
    #[gen_stub_pyfunction(module = "mini_numpy")]
    #[pyfunction]
    #[pyo3(signature=(start, stop=None, step=None))]
    /// It's arange - you know arange!
    fn arange(start: i32, stop: Option<i32>, step: Option<i32>) -> PyResult<PyVector> {
        Ok(PyVector {
            data: VectorData::Int(Tensor::arange(start, stop, step)?),
        })
    }
    #[gen_stub_pyfunction(module = "mini_numpy")]
    #[pyfunction]
    fn ones(shape: Vec<usize>) -> PyResult<PyVector> {
        Ok(PyVector {
            data: VectorData::Int(Tensor::ones(&shape)?),
        })
    }

    #[gen_stub_pyfunction(module = "mini_numpy")]
    #[pyfunction]
    fn zeros(shape: Vec<usize>) -> PyResult<PyVector> {
        Ok(PyVector {
            data: VectorData::Int(Tensor::zeros(&shape)?),
        })
    }

    #[gen_stub_pyfunction(module = "mini_numpy")]
    #[pyfunction]
    fn full(value: Bound<'_, PyAny>, shape: Vec<usize>) -> PyResult<PyVector> {
        if let Ok(value) = value.extract::<i32>() {
            Ok(PyVector {
                data: VectorData::Int(Tensor::full(value, &shape)?),
            })
        } else if let Ok(value) = value.extract::<f32>() {
            Ok(PyVector {
                data: VectorData::Float(Tensor::full(value, &shape)?),
            })
        } else {
            Err(PyTypeError::new_err("Type not supported!"))
        }
    }
}
define_stub_info_gatherer!(stub_info);
