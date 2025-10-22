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

# Pinned versions for Rust development tools
TARPAULIN_VERSION=0.32.0
NEXTEST_VERSION=0.9.101
AUDIT_VERSION=0.21.2
CAREFUL_VERSION=0.4.8

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

.PHONY: run-examples
run-examples: build ## Run the Rust examples
	@echo "Running Rust examples..."
	@cargo run --example quadtree
	@cargo run --example octree
	@cargo run --example kdtree
	@cargo run --example rtree
	@cargo run --example rstar_tree

.PHONY: run-py-examples
run-py-examples: develop-py ## Run the Python examples
	@echo "Running Python examples..."
	@bash -c "source .venv/bin/activate && python pyspart/examples/quadtree.py"
	@bash -c "source .venv/bin/activate && python pyspart/examples/octree.py"
	@bash -c "source .venv/bin/activate && python pyspart/examples/kdtree.py"
	@bash -c "source .venv/bin/activate && python pyspart/examples/rtree.py"
	@bash -c "source .venv/bin/activate && python pyspart/examples/rstar_tree.py"

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(WHEEL_DIR) dist/ $(PYSPART_DIR)/$(WHEEL_DIR) $(PYSPART_DIR)/*.so $(PYSPART_DIR)/target

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
	# Install each tool with a specific, pinned version
	@cargo install cargo-tarpaulin --version ${TARPAULIN_VERSION}
	@cargo install cargo-nextest --version ${NEXTEST_VERSION}
	@cargo install cargo-audit --version ${AUDIT_VERSION}
	@cargo install cargo-careful --version ${CAREFUL_VERSION}
	@sudo apt-get install -y python3-pip
	@pip install $(PY_DEP_MNGR)

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	@DEBUG_SPART=$(DEBUG_SPART) cargo clippy -- -D warnings -D clippy::unwrap_used -D clippy::expect_used

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

.PHONY: careful
careful: ## Run security checks on Rust code
	@echo "Running security checks..."
	@DEBUG_SPART=$(DEBUG_SPART) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo careful run

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
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets --workspace --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used

########################################################################################
## Python targets
########################################################################################

.PHONY: develop-py
develop-py: ## Build and install PySpart in the current Python environment
	@echo "Building and installing PySpart..."
	# Note: Maturin does not work when CONDA_PREFIX and VIRTUAL_ENV are both set
	@bash -c "source .venv/bin/activate && cd $(PYSPART_DIR) && unset CONDA_PREFIX && maturin develop"

.PHONY: wheel
wheel: ## Build the wheel file for PySpart
	@echo "Building the PySpart wheel..."
	@(cd $(PYSPART_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check)

.PHONY: wheel-manylinux
wheel-manylinux: ## Build the manylinux wheel file for PySpart (using Zig)
	@echo "Building the `manylinux` PySpart wheel..."
	@(cd $(PYSPART_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check --zig)

.PHONY: test-py
test-py: develop-py ## Run Python tests
	@echo "Running Python tests..."
	@bash -c "source .venv/bin/activate && pytest"

.PHONY: publish-py
publish-py: wheel-manylinux ## Publish the PySpart wheel to PyPI (requires PYPI_TOKEN to be set)
	@echo "Publishing PySpart to PyPI..."
	@if [ -z "$(WHEEL_FILE)" ]; then \
	   echo "Error: No wheel file found. Please run 'make wheel' first."; \
	   exit 1; \
	fi
	@echo "Found wheel file: $(WHEEL_FILE)"
	@twine upload -u __token__ -p $(PYPI_TOKEN) $(WHEEL_FILE)

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
