# Variables
PROJ_REPO := github.com/habedi/spart
BINARY_NAME := $(or $(PROJ_BINARY), $(notdir $(PROJ_REPO)))
BINARY = :target/release/$(BINARY_NAME)
PATH := /snap/bin:$(PATH)
RUST_BACKTRACE := 0
DEBUG_SPART := 0

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; \
 	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

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

.PHONY: doc
doc: format ## Generate the documentation
	@echo "Generating documentation..."
	@cargo doc --no-deps --document-private-items

.PHONY: fix-lint
fix-lint: ## Fix the linter warnings
	@echo "Fixing linter warnings..."
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets --workspace --all-features -- -D warnings
