/// Defines operations on Vectors (tensors - need to cleanup the naming throught the project)
pub mod vector_ops {
    // TODOS:
    // TODO: fancy indexing
    // TODO: Shape implementation, broadcasting rules
    // TODO: housekeeping: called many things vectors, this has ended up being a tensor based project
    // TODO: add more interesting operations (norms, Mat Mul, zeros, ones, random, etc.)
    // TODO: add testing suite
    use std::{
        fmt,
        ops::{Index, IndexMut},
    };

    use crate::errors::VectorError;

    /// Tensor Type - currently only supports numbers
    /// Inputs are data an optionally a shape
    /// If no shape is provided, the data is assumed to be flat
    pub struct Tensor<T> {
        data: Vec<T>,
        shape: Vec<usize>,
        strides: Vec<usize>,
    }

    impl<T> Tensor<T> {
        /// Tensor type
        /// data: base data to use
        /// shape: Optional shape info, otherwise extrapolated from data
        /// TODO: dtype
        pub fn new(data: Vec<T>, shape: Option<Vec<usize>>) -> Tensor<T> {
            match shape {
                Some(shape) => {
                    if data.len() != shape.iter().product::<usize>() {
                        // TODO this shouldn't panic
                        panic!()
                    }
                    Tensor {
                        data,
                        shape: shape.clone(),
                        strides: compute_strides(&shape),
                    }
                }
                None => {
                    let shape = vec![data.len()];
                    Tensor {
                        data,
                        shape,
                        strides: vec![1], // for 1d tensor, strides don't need to be defined
                    }
                }
            }
        }
    }

    impl<T> IndexMut<usize> for Tensor<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<T> Index<usize> for Tensor<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<T: Copy> Tensor<T> {
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
                strides: self.strides.clone(), // incorrect
            })
        }
    }

    impl<T: Eq> Tensor<T> {
        /// Checks if 2 arrays are equal, assuming that's well defined
        pub fn is_equal(&self, other: &Tensor<T>) -> bool {
            if self.data.len() != other.data.len() {
                return false;
            }
            let self_iter = self.data.iter();
            let other_iter = other.data.iter();

            self_iter.zip(other_iter).all(|(a, b)| a == b)
        }
    }

    /// Display array
    impl<T: ToString + Copy> fmt::Display for Tensor<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let num_dims = self.shape.len();
            let mut output_str = "[".repeat(num_dims);
            let mut index: Vec<usize> = vec![0; num_dims];
            let total_elements: usize = self.shape.iter().product();

            for _i in 0..total_elements {
                let mut offset: usize = 0;
                for (axis, stride) in std::iter::zip(&index, &self.strides) {
                    offset += axis * stride;
                }
                {
                    let data = self.data[offset];
                    output_str.push_str(&data.to_string());
                }

                // update offset state
                for i in (0..num_dims).rev() {
                    index[i] += 1;
                    if index[i] < self.shape[i] {
                        break;
                    } else {
                        index[i] = 0;
                    }
                }

                // update string
                // TODO: this might be doable within the update step
                let mut encountered_unfinished = false;
                let mut parens_closed = 0;
                for i in (0..num_dims).rev() {
                    let cur_idx = index[i];
                    match (cur_idx, encountered_unfinished) {
                        (0, false) => {
                            output_str.push(']');
                            parens_closed += 1;
                        }
                        (_, false) => {
                            encountered_unfinished = true;
                            output_str.push_str(", ");
                        }
                        (_, true) => {}
                    }
                }
                if index.iter().sum::<usize>() != 0 && parens_closed > 0 {
                    output_str.push('\n');
                    output_str.push_str(&" ".repeat(num_dims - parens_closed));
                    output_str.push_str(&"[".repeat(parens_closed))
                }
            }
            write!(f, "{output_str}")
        }
    }

    /// given a shape array, compute the stride
    fn compute_strides(shape: &[usize]) -> Vec<usize> {
        if shape.len() == 1 {
            return vec![1];
        }

        let mut strides: Vec<usize> = Vec::with_capacity(shape.len());
        strides.push(1);
        let shape_iter = shape.iter().skip(1).rev();
        for shape in shape_iter {
            strides.push(shape * strides[strides.len() - 1])
        }

        strides.reverse();
        strides
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_creation() {
            let data = vec![1, 2, 3];
            let tensor = Tensor::new(data, None);

            assert_eq!(tensor.data, vec![1, 2, 3]);
            assert_eq!(tensor.shape, vec![3]);
        }

        #[test]
        fn test_3d_stride() {
            let shape: Vec<usize> = vec![1, 3, 3];

            let strides = compute_strides(&shape);

            assert_eq!(strides, vec!(9, 3, 1))
        }

        #[test]
        fn test_4d_stride() {
            let shape: Vec<usize> = vec![1, 2, 3, 4];
            let strides = compute_strides(&shape);

            assert_eq!(strides, vec!(24, 12, 4, 1))
        }
    }
}
