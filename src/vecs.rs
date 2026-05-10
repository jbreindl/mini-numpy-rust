pub mod vector_ops {
    use num_traits::NumOps;

    pub struct NumericVector<T: NumOps>(Vec<T>);

    impl<T: NumOps> NumericVector<T> {
        pub fn new(data: Vec<T>) -> NumericVector<T> {
            NumericVector(data)
        }
    }
    impl<T: NumOps + Copy> NumericVector<T> {
        /// Accepts another array and a function f
        pub fn array_arithmetic(
            &self,
            other: &NumericVector<T>,
            f: fn(&T, &T) -> T,
        ) -> NumericVector<T> {
            let self_iter = self.0.iter();
            let other_iter = other.0.iter();
            // map arithmetic function onto both iters
            let result = self_iter
                .zip(other_iter)
                .map(|(a, b)| f(a, b))
                .collect::<Vec<_>>();
            NumericVector(result)
        }
    }

    impl<T: NumOps + Eq> NumericVector<T> {
        pub fn is_equal(&self, other: &NumericVector<T>) -> bool {
            let self_iter = self.0.iter();
            let other_iter = other.0.iter();

            self_iter.zip(other_iter).all(|(a, b)| a == b)
        }
    }

    impl<T: NumOps + ToString> ToString for NumericVector<T> {
        fn to_string(&self) -> String {
            let output = self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", output)
        }
    }
}
