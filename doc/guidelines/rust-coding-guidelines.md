# Coding Guidelines for Rust

## Coding Style
- Always use idiomatic rust
- Apply clean code rules

## Critical: Workspace Roots
- **Rust**: workspace root is `src/`, not the repo root. Always run `cargo` from `src/`.

## Building with Cargo (from `src/`)
```cmd
cargo build --workspace
```

## Testing
- Unit tests live alongside source files, run with `cargo test --workspace`
- All tests are unit tests co-located with source in `#[cfg(test)]`. No integration test files.
- Unit tests co-located with source in `#[cfg(test)]`

### Runnign tests with Cargo (from `src/`)
```cmd
cargo test --workspace
cargo test --workspace <test_name>        # run a single test by name substring
```
