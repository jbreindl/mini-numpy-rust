/// Defines operations on Vectors (tensors - need to cleanup the naming throught the project)
pub mod vector_ops {
    // TODOS:
    // TODO: Shape implementation, broadcasting rules
    // TODO: housekeeping: called many things vectors, this has ended up being a tensor based project
    // TODO: housekeeping: print function is gonna get nasty for big arrays
    // TODO: add more interesting operations (norms, Mat Mul, zeros, ones, random, etc.)
    // TODO: add testing suite
    use std::ops::{Index, IndexMut};

    use crate::errors::VectorError;
    use num_traits::NumOps;

    /// Tensor Type - currently only supports numbers
    /// Inputs are data an optionally a shape
    /// If no shape is provided, the data is assumed to be flat
    pub struct Tensor<T: NumOps> {
        data: Vec<T>,
        shape: Vec<usize>,
    }

    impl<T: NumOps> Tensor<T> {
        pub fn new(data: Vec<T>, shape: Option<Vec<usize>>) -> Tensor<T> {
            match shape {
                Some(shape) => Tensor { data, shape },
                None => {
                    let shape = vec![data.len()];
                    Tensor { data, shape }
                }
            }
        }
    }
    impl<T: NumOps> IndexMut<usize> for Tensor<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<T: NumOps> Index<usize> for Tensor<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T: NumOps + Copy> Tensor<T> {
        /// Accepts another array and a function f, that can be called pairwise
        pub fn array_arithmetic(
            &self,
            other: &Tensor<T>,
            f: fn(&T, &T) -> T,
        ) -> Result<Tensor<T>, VectorError> {
            if self.data.len() != other.data.len() {
                return Err(VectorError::MismatchedLengthError(
                    self.data.len(),
                    other.data.len(),
                ));
            }
            let self_iter = self.data.iter();
            let other_iter = other.data.iter();
            // map arithmetic function onto both iters
            let result = self_iter
                .zip(other_iter)
                .map(|(a, b)| f(a, b))
                .collect::<Vec<_>>();
            Ok(Tensor {
                data: result,
                shape: self.shape.clone(),
            })
        }
    }

    // Checks if 2 arraysare equal, assuming that's well defined
    impl<T: NumOps + Eq> Tensor<T> {
        pub fn is_equal(&self, other: &Tensor<T>) -> bool {
            if self.data.len() != other.data.len() {
                return false;
            }
            let self_iter = self.data.iter();
            let other_iter = other.data.iter();

            self_iter.zip(other_iter).all(|(a, b)| a == b)
        }
    }

    // ability to print the array
    impl<T: NumOps + ToString> ToString for Tensor<T> {
        fn to_string(&self) -> String {
            let output = self
                .data
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", output)
        }
    }
}
