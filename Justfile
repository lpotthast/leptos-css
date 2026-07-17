# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

# Perform the one-time local setup required for this crate.
once:
  just enable-wasm
  just install-tools

# Install the tools this crate depends on for local development.
install-tools:
  cargo install just
  cargo install cargo-audit
  cargo install cargo-deny
  cargo install cargo-semver-checks

# Enable the WASM target required by the wasm compile check.
enable-wasm:
  rustup target add wasm32-unknown-unknown

# Format the crate.
fmt:
  cargo fmt --all

# Run format checks.
fmt-check:
  cargo fmt --all --check

# Run clippy.
clippy:
  cargo clippy --all-targets -- -D warnings

# Type-check the crate.
check:
  cargo check --all-targets

# Verify the library compiles for the wasm32-unknown-unknown target.
check-wasm:
  cargo check --target wasm32-unknown-unknown --locked

# Run unit tests, integration tests, and doctests for the native target.
test:
  cargo test

# Run only the crate unit tests.
test-lib:
  cargo test --lib

# Run only the crate doc tests.
test-doc:
  cargo test --doc

# Build documentation with rustdoc warnings denied (matches CI).
doc:
  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked

# Scan Cargo.lock against the RustSec advisory database.
audit:
  cargo audit

# Run cargo-deny's supply-chain checks (advisories, bans, sources).
deny:
  cargo deny check

# Detect breaking public-API changes vs. the latest published release.
semver-check:
  cargo semver-checks

# Clean build artifacts for this crate.
clean:
  cargo clean

# Run the most important verification commands for this crate.
verify:
  just fmt-check
  just check
  just check-wasm
  just clippy
  just test
  just doc
