# Codebase Map

## Areas

- `crates/`: Cargo workspace, one crate per directory; each crate is also a moon project.
- `scripts/`: `bootstrap.sh` (toolchain and dependency install), `check-dependency-rule.sh`.
- `.moon/`: workspace, toolchains and the tasks every Rust or JavaScript project inherits.
- `.claude/rules/`: coding rules scoped by path.

## Packages

- `crates/shared-kernel`: typed ids and `Redacted<T>`, shared by every context.
