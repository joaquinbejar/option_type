# =============================================================================
# Makefile for option_type
# Option contract type definitions including exotic options
# =============================================================================

# Detect current branch
CURRENT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD)

# Project name for packaging
PROJECT_NAME := option_type

# =============================================================================
# Default target
# =============================================================================
.PHONY: all
all: fmt lint test build ## Run fmt, lint, test, and build (default target)

# =============================================================================
# 🔧 Build & Run
# =============================================================================

.PHONY: build
build: ## Build debug version
	@echo "🔨 Building debug version..."
	cargo build

.PHONY: release
release: ## Build release version
	@echo "🚀 Building release version..."
	cargo build --release

.PHONY: clean
clean: ## Remove build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# =============================================================================
# 🧪 Test & Quality
# =============================================================================

.PHONY: test
test: ## Run all tests with all features enabled
	@echo "🧪 Running all tests..."
	RUST_LOG=warn cargo test --all-features

.PHONY: test-default
test-default: ## Run all tests with default (no) features enabled
	@echo "🧪 Running tests with default features (no-default-features)..."
	RUST_LOG=warn cargo test --no-default-features

.PHONY: test-lib
test-lib: ## Run library unit tests only
	@echo "🧪 Running library tests..."
	RUST_LOG=warn cargo test --lib

.PHONY: test-doc
test-doc: ## Run documentation tests only
	@echo "🧪 Running documentation tests..."
	cargo test --doc

.PHONY: fmt
fmt: ## Format code with rustfmt
	@echo "✨ Formatting code..."
	cargo +stable fmt --all

.PHONY: fmt-check
fmt-check: ## Check code formatting without modifying files
	@echo "🔍 Checking code formatting..."
	cargo +stable fmt --all --check

.PHONY: lint
lint: ## Run clippy lints (all-targets, all-features, deny warnings)
	@echo "🔍 Running clippy lints..."
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-fix
lint-fix: ## Auto-fix lint issues via clippy --fix
	@echo "🔧 Auto-fixing lint issues..."
	cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged -- -D warnings

.PHONY: fix
fix: ## Apply cargo fix suggestions
	@echo "🔧 Applying cargo fix suggestions..."
	cargo fix --allow-staged --allow-dirty

.PHONY: check
check: fmt-check lint test ## Run fmt-check, lint, and test
	@echo "✅ All checks passed!"

.PHONY: pre-push
pre-push: fmt-check lint test test-default release doc-check ## Run the full CI-equivalent gate (pure check, no auto-fixing)
	@echo "✅ All pre-push checks passed!"

# =============================================================================
# 📦 Packaging & Docs
# =============================================================================

.PHONY: doc
doc: ## Generate documentation (private items included)
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --document-private-items

.PHONY: doc-check
doc-check: ## Build docs with all features, denying warnings (matches CI)
	@echo "📚 Checking documentation build (deny warnings, all-features)..."
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

.PHONY: doc-open
doc-open: ## Open generated documentation in browser
	@echo "📚 Opening documentation in browser..."
	cargo doc --no-deps --open

.PHONY: publish-dry
publish-dry: ## Dry-run cargo publish (no upload, safe to re-run)
	@echo "📦 Dry-run publishing to crates.io..."
	cargo publish --dry-run
	@echo "Dry run complete. Run 'make publish' to actually publish."

.PHONY: publish
publish: ## Publish to crates.io for real (requires CARGO_REGISTRY_TOKEN)
	@echo "📦 Publishing to crates.io..."
	@if [ -z "$$CARGO_REGISTRY_TOKEN" ]; then \
		echo "❌ CARGO_REGISTRY_TOKEN is not set in the environment. Aborting."; \
		exit 1; \
	fi
	cargo publish

# =============================================================================
# 📈 Coverage
# =============================================================================

.PHONY: coverage
coverage: ## Generate code coverage report (XML via tarpaulin, all-features)
	@echo "📊 Generating code coverage report (XML)..."
	@command -v cargo-tarpaulin > /dev/null || cargo install cargo-tarpaulin
	@mkdir -p coverage
	RUST_LOG=warn cargo tarpaulin --verbose --all-features --timeout 120 --out Xml --output-dir coverage

# =============================================================================
# 🚀 Release
# =============================================================================

.PHONY: version
version: ## Print the current crate version from Cargo.toml
	@echo "📋 Current version:"
	@grep '^version' Cargo.toml | head -1

.PHONY: tag
tag: ## Create a git tag from the version in Cargo.toml
	@echo "🏷️  Creating git tag..."
	@version=$$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'); \
	git tag -a "v$$version" -m "Release v$$version"; \
	echo "Created tag v$$version"

# =============================================================================
# ℹ️  Help
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
