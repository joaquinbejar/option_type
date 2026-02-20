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
all: fmt lint test build

# =============================================================================
# 🔧 Build & Run
# =============================================================================

.PHONY: build
build:
	@echo "🔨 Building debug version..."
	cargo build

.PHONY: release
release:
	@echo "🚀 Building release version..."
	cargo build --release

.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# =============================================================================
# 🧪 Test & Quality
# =============================================================================

.PHONY: test
test:
	@echo "🧪 Running all tests..."
	RUST_LOG=warn cargo test --all-features

.PHONY: test-lib
test-lib:
	@echo "🧪 Running library tests..."
	RUST_LOG=warn cargo test --lib

.PHONY: test-doc
test-doc:
	@echo "🧪 Running documentation tests..."
	cargo test --doc

.PHONY: fmt
fmt:
	@echo "✨ Formatting code..."
	cargo +stable fmt --all

.PHONY: fmt-check
fmt-check:
	@echo "🔍 Checking code formatting..."
	cargo +stable fmt --all --check

.PHONY: lint
lint:
	@echo "🔍 Running clippy lints..."
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-fix
lint-fix:
	@echo "🔧 Auto-fixing lint issues..."
	cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged -- -D warnings

.PHONY: fix
fix:
	@echo "🔧 Applying cargo fix suggestions..."
	cargo fix --allow-staged --allow-dirty

.PHONY: check
check: fmt-check lint test
	@echo "✅ All checks passed!"

.PHONY: pre-push
pre-push: fix fmt lint-fix test doc
	@echo "✅ All pre-push checks passed!"

# =============================================================================
# 📦 Packaging & Docs
# =============================================================================

.PHONY: doc
doc:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --document-private-items

.PHONY: doc-open
doc-open:
	@echo "📚 Opening documentation in browser..."
	cargo doc --no-deps --open

.PHONY: publish
publish:
	@echo "📦 Publishing to crates.io..."
	cargo publish --dry-run
	@echo "Dry run complete. Run 'cargo publish' to actually publish."

# =============================================================================
# 📈 Coverage
# =============================================================================

.PHONY: coverage
coverage:
	@echo "📊 Generating code coverage report (XML)..."
	@command -v cargo-tarpaulin > /dev/null || cargo install cargo-tarpaulin
	@mkdir -p coverage
	RUST_LOG=warn cargo tarpaulin --verbose --all-features --timeout 120 --out Xml --output-dir coverage

# =============================================================================
# 🚀 Release
# =============================================================================

.PHONY: version
version:
	@echo "📋 Current version:"
	@grep '^version' Cargo.toml | head -1

.PHONY: tag
tag:
	@echo "🏷️  Creating git tag..."
	@version=$$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'); \
	git tag -a "v$$version" -m "Release v$$version"; \
	echo "Created tag v$$version"
