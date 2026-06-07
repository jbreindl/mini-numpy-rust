build:
    uv run maturin dev

test: build
    uv run pytest


