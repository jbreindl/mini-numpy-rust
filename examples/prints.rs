fn main() {
    // 2d
    use mini_numpy::vecs::vector_ops;
    {
        println!("2d Tensor:");
        let shape: Vec<usize> = vec![2, 3];
        let mut tensor = vector_ops::Tensor::new((0..6).collect(), Some(shape));
        println!("{tensor}");

        println!("2d after transpose");
        tensor.transpose();
        println!("{tensor}")
    }
    // 3d
    {
        println!("3d Tensor:");
        let shape: Vec<usize> = vec![2, 2, 2];
        let mut tensor = vector_ops::Tensor::new((0..8).collect(), Some(shape));
        println!("{tensor}");

        println!("3d after transpose");
        tensor.transpose();
        println!("{tensor}");
    }
    // 4d
    {
        println!("4d Tensor:");
        let shape: Vec<usize> = vec![2, 2, 2, 2];
        let mut tensor = vector_ops::Tensor::new((0..16).collect(), Some(shape));
        println!("{tensor}");

        println!("4d after transpose");
        tensor.transpose();
        println!("{tensor}")
    }
}
