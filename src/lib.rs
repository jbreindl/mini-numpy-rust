use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {
    use pyo3::{exceptions::PyValueError, prelude::*};

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn multiply_numbers(a: usize, b: usize) -> PyResult<usize> {
        Ok(a * b)
    }

    /// TODO: want to extend this to be generically numeric, will do later
    #[pyclass(sequence)]
    struct MyVector(Vec<i32>);

    #[pymethods]
    impl MyVector {
        #[new]
        fn new(value: Vec<i32>) -> Self {
            MyVector(value)
        }

        fn __repr__(&self) -> String {
            let values = self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            format!("[{}]", values)
        }

        /// TODO: add a scalar
        fn __add__(&self, other: &MyVector) -> PyResult<MyVector> {
            if self.0.len() != other.0.len() {
                return Err(PyValueError::new_err("Lengths must match"));
            }

            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let summed = self_iter.zip(other_iter).map(|(a, b)| a + b).collect();

            Ok(MyVector(summed))
        }

        fn __mul__(&self, other: &MyVector) -> PyResult<MyVector> {
            if self.0.len() != other.0.len() {
                return Err(PyValueError::new_err("Lengths must match"));
            }

            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let multiplied = self_iter.zip(other_iter).map(|(a, b)| a * b).collect();

            Ok(MyVector(multiplied))
        }

        fn __sub__(&self, other: &MyVector) -> PyResult<MyVector> {
            if self.0.len() != other.0.len() {
                return Err(PyValueError::new_err("Lengths must match"));
            }

            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let subtracted = self_iter.zip(other_iter).map(|(a, b)| a - b).collect();

            Ok(MyVector(subtracted))
        }

        fn __truediv__(&self, other: &MyVector) -> PyResult<MyVector> {
            if self.0.len() != other.0.len() {
                return Err(PyValueError::new_err("Lengths must match"));
            }

            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let divided = self_iter.zip(other_iter).map(|(a, b)| a / b).collect();

            Ok(MyVector(divided))
        }

        fn __eq__(&self, other: &MyVector) -> bool {
            if self.0.len() != other.0.len() {
                return false;
            }

            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let mut final_bool = true;

            for (a, b) in self_iter.zip(other_iter) {
                final_bool &= a == b;
            }
            final_bool
        }
    }
}
