.PHONY: build test install clean fmt check lint docs

# Build the project
build:
	cd thefuck && cargo build --release

# Build in debug mode
build-debug:
	cd thefuck && cargo build

# Run tests
test:
	cd thefuck && cargo test

test-verbose:
	cd thefuck && cargo test -- --nocapture

# Install to system
install: build
	cp thefuck/target/release/thefuck /usr/local/bin/
	cp thefuck/target/release/thefuck_firstuse /usr/local/bin/

# Format code
fmt:
	cd thefuck && cargo fmt

# Check code for issues
check:
	cd thefuck && cargo clippy -- -D warnings

# Run linter (ruff)
lint:
	@echo "Running ruff..."
	ruff check .
	ruff format .

# Generate documentation
docs:
	cd thefuck && cargo doc --open

# Clean build artifacts
clean:
	cd thefuck && cargo clean

# Development setup
dev-setup:
	@echo "Setting up development environment..."
	rustup update
	cargo install --locked cargo-watch
	cargo install --locked cargo-audit
	@echo "Run 'make install' to install thefuck"

# Watch for changes
watch:
	cargo watch -x "build"

# Run example commands
example:
	@echo "Running example corrections..."
	cd thefuck && cargo run -- gti status
	cd thefuck && cargo run -- python --version

# Check for security issues
audit:
	cd thefuck && cargo audit

# Update dependencies
update:
	cd thefuck && cargo update

# Build Windows binaries
windows:
	cd thefuck && cargo build --release --target x86_64-pc-windows-gnu
	mkdir -p dist
	cp thefuck/target/x86_64-pc-windows-gnu/release/thefuck.exe dist/
	cp thefuck/target/x86_64-pc-windows-gnu/release/thefuck_firstuse.exe dist/

# Create source distribution
dist: clean
	@echo "Creating distribution..."
	mkdir -p dist
	cp -r . dist/thefuck-upgrade
	cd dist && tar -czf thefuck-upgrade.tar.gz thefuck-upgrade

# Development helpers
check-all: fmt check lint test
setup-env: dev-setup
run: build-debug
debug: build-debug
release: build