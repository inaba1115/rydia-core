# ---------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------

# Python executable used by uv (for pyo3 / maturin)
PYTHON_EXE := $(shell uv run python -c "import sys; print(sys.executable)")
PYO3_ENV   := PYO3_PYTHON=$(PYTHON_EXE)

# Paths
PY_SRC_DIR := .
PY_TEST_DIR := tests
STUB_OUT_DIR := rydia
STUB_NAME := rydia.pyi

# ---------------------------------------------------------------------
# Meta targets
# ---------------------------------------------------------------------

.PHONY: all check dev
all: check
check: lint test
dev: develop stub

# ---------------------------------------------------------------------
# Python
# ---------------------------------------------------------------------

.PHONY: lint_py test_py

lint_py:
	uv run ruff check $(PY_SRC_DIR) --fix
	uv run ruff format $(PY_SRC_DIR)
	uv run ty check .

test_py:
	uv run pytest $(PY_TEST_DIR)

# ---------------------------------------------------------------------
# Rust
# ---------------------------------------------------------------------

.PHONY: lint_rs test_rs

lint_rs:
	cargo fmt

test_rs:
	$(PYO3_ENV) cargo test

# ---------------------------------------------------------------------
# Combined
# ---------------------------------------------------------------------

.PHONY: lint test

lint: lint_py lint_rs
test: test_py test_rs

# ---------------------------------------------------------------------
# Development
# ---------------------------------------------------------------------

.PHONY: develop
develop:
	maturin develop

# ---------------------------------------------------------------------
# Stub generation (internal use only)
# ---------------------------------------------------------------------

.PHONY: stub
stub:
	$(PYO3_ENV) cargo run --bin stub_gen
	mkdir -p $(STUB_OUT_DIR)
	mv rydia_core.pyi $(STUB_OUT_DIR)/$(STUB_NAME)

