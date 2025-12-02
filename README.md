# Advent of Code 2025

Solutions in Rust.

## Structure

```
├── common/          # Shared utilities
├── day01/           # Day 1 solution
├── day02/           # Day 2 solution
└── inputs/          # Input files (gitignored)
```

## Usage

```bash
# Run a day's solution
cargo run -p day01

# Run tests
cargo test -p day01

# Run all tests
cargo test --workspace

# Check formatting
cargo fmt --all

# Run clippy
cargo clippy --workspace

# Watch mode (requires: cargo install cargo-watch)
cargo watch -c -x 'run -p day01'           # Watch and run
cargo watch -c -x 'test -p day01'          # Watch and test
cargo watch -c -x 'test --workspace'       # Watch all tests
```

## Adding a New Day

1. Create `dayXX/Cargo.toml`:
```toml
[package]
name = "dayXX"
version.workspace = true
edition.workspace = true

[dependencies]
common = { path = "../common" }

```

2. Create `dayXX/src/main.rs` with solution
3. Add `"dayXX"` to workspace members in root `Cargo.toml`
4. Add input to `inputs/dayXX.txt`
