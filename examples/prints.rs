fn main() {
    // 2d
    use mini_numpy::vecs::vector_ops;
    {
        println!("2d Tensor:");
        let shape: Vec<usize> = vec![2, 3];
        let tensor = vector_ops::Tensor::new(vec![1, 2, 3, 4, 5, 6], Some(shape));
        println!("{tensor}")
    }
    // 3d
    {
        println!("3d Tensor:");
        let shape: Vec<usize> = vec![2, 2, 2];
        let tensor = vector_ops::Tensor::new(vec![1, 2, 3, 4, 5, 6, 7, 8], Some(shape));
        println!("{tensor}")
    }
}
