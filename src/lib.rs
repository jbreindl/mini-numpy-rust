use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mini_numpy {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn multiply_numbers(a: usize, b: usize) -> PyResult<usize>{
        Ok((a * b))
    }
    
}
