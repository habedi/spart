# Variables
PKG = github.com/habedi/template-rust-project
BINARY_NAME = $(or $(PROJ_BINARY), $(notdir $(PKG)))
BINARY = target/release/$(BINARY_NAME)
PATH := /snap/bin:$(PATH)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: format
format: ## Format Rust files
	@echo "Formatting Rust files..."
	cargo fmt

.PHONY: test
test: format ## Run tests
	@echo "Running tests..."
	cargo test -- --nocapture

.PHONY: coverage
coverage: format ## Generate test coverage report
	@echo "Generating test coverage report..."
	cargo tarpaulin --out Xml --out Html

.PHONY: build
build: format ## Build the binary for the current platform
	@echo "Building the project..."
	cargo build --release

.PHONY: run
run: build ## Build and run the binary
	@echo "Running the $(BINARY) binary..."
	./$(BINARY)

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	cargo clean

.PHONY: install-snap
install-snap: ## Install a few dependencies using Snapcraft
	@echo "Installing the snap package..."
	sudo apt-get update
	sudo apt-get install -y snapd
	sudo snap refresh
	sudo snap install rustup --classic

.PHONY: install-deps
install-deps: install-snap ## Install development dependencies
	@echo "Installing dependencies..."
	rustup component add rustfmt clippy
	cargo install cargo-tarpaulin

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	cargo clippy -- -D warnings

.PHONY: publish
publish: ## Publish the package to crates.io (requires CARGO_REGISTRY_TOKEN to be set)
	@echo "Publishing the package to Cargo registry..."
	cargo publish --token $(CARGO_REGISTRY_TOKEN)