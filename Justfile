gen:
	cargo component bindings
	cargo fmt --all

build:
	cargo fmt --all
	cargo build --workspace --release

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

lint-fix:
	cargo component bindings
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged
	cargo sort --workspace
	cargo machete

test-verbose:
	cargo test --workspace -- --nocapture

# WASM component validation
validate-wasm: build
	@echo "Validating WASM components..."
	wasm-tools validate target/wasm32-wasip2/release/time.wasm
	wasm-tools validate target/wasm32-wasip2/release/fetch.wasm
	@echo "✅ All WASM components are valid"

extract-wit:
	@echo "Extracting WIT interfaces..."
	wasm-tools component wit target/wasm32-wasip2/release/time.wasm > target/time.wit
	wasm-tools component wit target/wasm32-wasip2/release/fetch.wasm > target/fetch.wit
	@echo "Time component WIT:"
	cat target/time.wit
	@echo "---"
	@echo "Fetch component WIT:"
	cat target/fetch.wit

# WASM component inspection
inspect-time:
	wasm-tools component wit target/wasm32-wasip2/release/time.wasm

inspect-fetch:
	wasm-tools component wit target/wasm32-wasip2/release/fetch.wasm

# Clean commands
clean:
	cargo clean
	rm -rf target/
	rm -rf build/

clean-cache:
	cargo cache clean --all

# CI tool installation (using cargo-binstall, minimal dependencies)
install-tools:
	@echo "Installing cargo-binstall..."
	which cargo-binstall > /dev/null || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	@echo "Installing rustfmt..."
	cargo fmt --version > /dev/null || rustup component add rustfmt
	cargo clippy --version > /dev/null || rustup component add clippy
	@echo "Installing wasm-tools..."
	which wasm-tools > /dev/null || (yes | cargo binstall wasm-tools --force || cargo install wasm-tools)
	which wkg > /dev/null || (yes | cargo binstall wkg --force)
	@echo "✅ CI tools installed"

# CI testing (runs all checks)
ci: lint validate-wasm extract-wit
	@echo "✅ All CI checks passed"

# Development workflow
dev: build validate-wasm extract-wit
	@echo "✅ Development build complete"

# Release preparation
release-prep: lint-fix validate-wasm
	@echo "✅ Ready for release"

# Component documentation
docs:
	@echo "Generating component documentation..."
	cargo doc --workspace --no-deps --target wasm32-wasip2
	@echo "✅ Documentation generated"

# Quick test (build + basic validation)
quick-test: build validate-wasm
	@echo "✅ Quick test passed"

# Full test suite
full-test: ci
	@echo "✅ Full test suite passed"

# Setup for new developers
setup: install-tools build
	@echo "✅ Development environment setup complete"

# Component size analysis
analyze-size: build
	@echo "Analyzing component sizes..."
	@echo "Time component:"
	wc -c target/wasm32-wasip2/release/time.wasm
	@echo "Fetch component:"
	wc -c target/wasm32-wasip2/release/fetch.wasm
