pub mod vector_ops {
    use num_traits::NumOps;
    use std::fmt;

    pub struct NumericVector<T: NumOps>(Vec<T>);

    impl<T: NumOps + ToString> NumericVector<T> {
        pub fn new(data: Vec<T>) -> NumericVector<T> {
            NumericVector(data)
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
