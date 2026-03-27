# Run all tests
test:
    cargo test

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Generate HTML coverage report and open in browser
cover:
    cargo llvm-cov --html --open

# Print coverage summary to terminal
cover-summary:
    cargo llvm-cov
