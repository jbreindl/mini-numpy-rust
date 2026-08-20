build:
    uv run maturin dev
    cargo run --features="pyo3/generate-import-lib" --bin stub_gen    



test_rust: build
    cargo test

test_python: 
    uv run pytest

test:
    just test_rust
    just test_python
