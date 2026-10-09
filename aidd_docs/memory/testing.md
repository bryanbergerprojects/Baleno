# Testing

## Strategy

- Rust: unit tests in a `#[cfg(test)] mod tests` inside the module, plus doctests (including `compile_fail` ones that prove a type cannot do something).
- Real SQLite in tests, no repository mocks.

## Tools

- cargo-nextest; doctests run in a second `cargo test --doc` pass because nextest skips them.

## Run

- `moon run :test`, or `moon run <crate>:test` for one crate.
