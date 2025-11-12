
```makefile
# TLP Makefile

#build
build:
	cargo build

#build CLI
release:
	cargo build --release

#run cli
run:
	cargo run

#parse a specific file
parse:
	cargo run -- parse $(FILE)

#run tests
test:
	cargo test

#format code
fmt:
	cargo fmt

#lint code
clippy:
	cargo clippy

#run everything before commit
pre-commit: fmt clippy test

#generate documentation
doc:
	cargo doc --open

#clean build artifacts
clean:
	cargo clean

#install dependencies
install:
	cargo fetch

#run benchmarks
bench:
	cargo bench

#show help
help:
	@echo "Tiny Language Parser - Available Targets:"
	@echo ""
	@echo "  build       - Build the project"
	@echo "  release     - Build in release mode"
	@echo "  run         - Run the CLI"
	@echo "  parse FILE= - Parse a specific file"
	@echo "  test        - Run tests"
	@echo "  fmt         - Format code"
	@echo "  clippy      - Lint code"
	@echo "  pre-commit  - Run fmt, clippy, and test (before commit)"
	@echo "  doc         - Generate and open documentation"
	@echo "  clean       - Clean build artifacts"
	@echo "  install     - Install dependencies"
	@echo "  bench       - Run benchmarks"
	@echo "  help        - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make parse FILE=test_data/powers.txt"
	@echo "  make pre-commit"

.PHONY: build release run parse test fmt clippy pre-commit doc clean install bench help