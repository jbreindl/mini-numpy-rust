use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // `stub_info` is defined by `define_stub_info_gatherer!`
    let stub = mini_numpy::stub_info()?;
    stub.generate()?;
    Ok(())
}
