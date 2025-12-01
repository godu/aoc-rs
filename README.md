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

# Run benchmarks
cargo bench -p day01

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

[dev-dependencies]
criterion.workspace = true

[[bench]]
name = "bench"
harness = false
```

2. Create `dayXX/src/main.rs` with solution
3. Create `dayXX/benches/bench.rs` for benchmarks
4. Add `"dayXX"` to workspace members in root `Cargo.toml`
5. Add input to `inputs/dayXX.txt`
