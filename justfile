build:
    uv run maturin dev
    cargo run --features="pyo3/generate-import-lib" --bin stub_gen    

test: build
    uv run pytest


