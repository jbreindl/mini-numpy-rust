use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {
    use std::iter;

    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn multiply_numbers(a: usize, b: usize) -> PyResult<usize> {
        Ok(a * b)
    }

    /// want to extend this to be generically numeric, will do later
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

        fn __add__(&self, other: &MyVector) -> MyVector {
            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            let summed = self_iter.zip(other_iter).map(|(a, b)| a + b).collect();

            MyVector(summed)
        }
    }
}
