# PaneerLang Makefile

.PHONY: build run test clean install examples help

# Default target
all: build

# Build the project
build:
	cargo build --release

# Build in debug mode
debug:
	cargo build

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install dependencies
deps:
	cargo fetch

# Check code without building
check:
	cargo check

# Run clippy for linting
lint:
	cargo clippy

# Format code
fmt:
	cargo fmt

# Run the main example
run-example:
	cargo run example.paneer

# Run calculator example
run-calculator:
	cargo run examples/calculator.paneer

# Run strings example
run-strings:
	cargo run examples/strings.paneer

# Run test file
run-test:
	cargo run test.paneer

# Start REPL
repl:
	cargo run -- --repl

# Install the binary
install: build
	cp target/release/paneerlang /usr/local/bin/

# Run all examples
examples: run-example run-calculator run-strings run-test

# Show help
help:
	@echo "PaneerLang Build System"
	@echo "======================="
	@echo ""
	@echo "Available targets:"
	@echo "  build          - Build release version"
	@echo "  debug          - Build debug version"
	@echo "  test           - Run tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  check          - Check code without building"
	@echo "  lint           - Run clippy linter"
	@echo "  fmt            - Format code"
	@echo "  run-example    - Run main example.paneer"
	@echo "  run-calculator - Run calculator example"
	@echo "  run-strings    - Run strings example"
	@echo "  run-test       - Run test.paneer"
	@echo "  repl           - Start interactive REPL"
	@echo "  examples       - Run all examples"
	@echo "  install        - Install binary to /usr/local/bin"
	@echo "  help           - Show this help"