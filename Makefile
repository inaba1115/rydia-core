PYO3_ENV := PYO3_PYTHON=$(shell uv run python -c "import sys; print(sys.executable)")

.PHONY: all
all: lint_py test_py lint_rs test_rs

.PHONY: lint_py
lint_py:
	# uv run ruff check . --fix
	# uv run ruff format .
	# uv run ty check .

.PHONY: test_py
test_py:
	uv run pytest tests

.PHONY: lint_rs
lint_rs:
	cargo fmt

.PHONY: test_rs
test_rs:
	$(PYO3_ENV) cargo test

.PHONY: develop
develop:
	# $(PYO3_ENV) cargo run --bin stub_gen
	# mv rydia.pyi rydia/
	$(PYO3_ENV) maturin develop
	# make lint
