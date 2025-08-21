# Variables
PROJ_REPO := github.com/habedi/spart
BINARY_NAME := $(or $(PROJ_BINARY), $(notdir $(PROJ_REPO)))
BINARY = :target/release/$(BINARY_NAME)
PATH := /snap/bin:$(PATH)
RUST_BACKTRACE := 0
DEBUG_SPART := 0
RUST_LOG        := info
WHEEL_DIR       := dist
PYSPART_DIR     := pyspart
PY_DEP_MNGR     := uv
WHEEL_FILE      := $(shell ls $(PYSPART_DIR)/$(WHEEL_DIR)/pyspart-*.whl 2>/dev/null | head -n 1)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show the help messages for all targets
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*## .*$$' Makefile | \
	awk 'BEGIN {FS = ":.*## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

.PHONY: format
format: ## Format Rust files
	@echo "Formatting Rust files..."
	@cargo fmt

.PHONY: test
test: format ## Run the tests
	@echo "Running tests..."
	@DEBUG_SPART=$(DEBUG_SPART) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo test -- --nocapture

.PHONY: coverage
coverage: format ## Generate test coverage report
	@echo "Generating test coverage report..."
	@DEBUG_SPART=$(DEBUG_SPART) cargo tarpaulin --out Xml --out Html

.PHONY: build
build: format ## Build the binary for the current platform
	@echo "Building the project..."
	@DEBUG_SPART=$(DEBUG_SPART) cargo build --release

.PHONY: run
run: build ## Build and run the binary
	@echo "Running the $(BINARY) binary..."
	@DEBUG_SPART=$(DEBUG_SPART) ./$(BINARY)

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(WHEEL_DIR) dist/ $(PYSPART_DIR)/$(WHEEL_DIR)
	@rm -f $(PYSPART_DIR)/*.so

.PHONY: install-snap
install-snap: ## Install a few dependencies using Snapcraft
	@echo "Installing the snap package..."
	@sudo apt-get update
	@sudo apt-get install -y snapd
	@sudo snap refresh
	@sudo snap install rustup --classic

.PHONY: install-deps
install-deps: install-snap ## Install development dependencies
	@echo "Installing dependencies..."
	@rustup component add rustfmt clippy
	@cargo install cargo-tarpaulin
	@cargo install cargo-audit
	@cargo install nextest
	@sudo apt-get install python3-pip
	@pip install $(PY_DEP_MNGR)

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	@DEBUG_SPART=$(DEBUG_SPART) cargo clippy -- -D warnings

.PHONY: publish
publish: ## Publish the package to crates.io (requires CARGO_REGISTRY_TOKEN to be set)
	@echo "Publishing the package to Cargo registry..."
	@cargo publish --token $(CARGO_REGISTRY_TOKEN)

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	@DEBUG_SPART=$(DEBUG_SPART) cargo bench

.PHONY: audit
audit: ## Run security audit on Rust dependencies
	@echo "Running security audit..."
	@cargo audit

.PHONY: nextest
nextest: ## Run tests using nextest
	@echo "Running tests using nextest..."
	@DEBUG_SPART=$(DEBUG_SPART) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo nextest run

.PHONY: docs
docs: format ## Generate the documentation
	@echo "Generating documentation..."
	@cargo doc --no-deps --document-private-items

.PHONY: fix-lint
fix-lint: ## Fix the linter warnings
	@echo "Fixing linter warnings..."
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets --workspace --all-features -- -D warnings

########################################################################################
## Python targets
########################################################################################

.PHONY: develop-py
develop-py: ## Build and install PySpart in the current Python environment
	@echo "Building and installing PySpart..."
	# Note: Maturin does not work when CONDA_PREFIX and VIRTUAL_ENV are both set
	@(cd $(PYSPART_DIR) && unset CONDA_PREFIX && maturin develop)

.PHONY: wheel
wheel: ## Build the wheel file for PySpart
	@echo "Building the PySpart wheel..."
	@(cd $(PYSPART_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check)

.PHONY: wheel-manylinux
wheel-manylinux: ## Build the manylinux wheel file for PySpart (using Zig)
	@echo "Building the manylinux PySpart wheel..."
	@(cd $(PYSPART_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check --zig)

.PHONY: test-py
test-py: develop-py ## Run Python tests
	@echo "Running Python tests..."
	@$(PY_DEP_MNGR) run pytest

.PHONY: publish-py
publish-py: wheel-manylinux ## Publish the PySpart wheel to PyPI (requires PYPI_TOKEN to be set)
	@echo "Publishing PySpart to PyPI..."
	@if [ -z "$(WHEEL_FILE)" ]; then \
	   echo "Error: No wheel file found. Please run 'make wheel' first."; \
	   exit 1; \
	fi
	@echo "Found wheel file: $(WHEEL_FILE)"
	@twine upload -u __token__ -p $(PYPI_TOKEN) $(WHEEL_FILE)

.PHONY: generate-ci
generate-ci: ## Generate CI configuration files (GitHub Actions workflow)
	@echo "Generating CI configuration files..."
	@(cd $(PYSPART_DIR) && maturin generate-ci --zig --pytest --platform all -o ../.github/workflows/ci.yml github)

########################################################################################
## Additional targets
########################################################################################

.PHONY: setup-hooks
setup-hooks: ## Install Git hooks (pre-commit and pre-push)
	@echo "Installing Git hooks..."
	@pre-commit install --hook-type pre-commit
	@pre-commit install --hook-type pre-push
	@pre-commit install-hooks

.PHONY: test-hooks
test-hooks: ## Test Git hooks on all files
	@echo "Testing Git hooks..."
	@pre-commit run --all-files
