# File Organization Conventions

This document defines the file layout and naming rules for this project. rustr is a flat-layout crate: every source file sits directly under `src/`, and the test suite is a pair of top-level test crates. The layout stays flat while it fits the budget in [`PATTERNS.md`](PATTERNS.md) under "Flat layout allowed only when trivial". A concern that outgrows a single file moves into a `src/<concern>/` directory with a thin `mod.rs`, following the multi-file module shape described there.

## Source Files

Each file at `src/` root owns one concern.

| File | Role |
|---|---|
| `main.rs` | Binary entry point. Parses arguments, resolves the target project, and dispatches to one action. |
| `lib.rs` | Library root. Declares every module with `pub mod` and contains nothing else. |
| `cli.rs` | `CliArgs` and the argument parser (`parse_args_from`, `parse_args`). |
| `constants.rs` | `pub const` declarations only: the cargo command, file and directory names, the projects-directory environment variable, and the platform binary extension. |
| `helpers.rs` | Project discovery, `Cargo.toml` interpretation, cargo invocation, binary copying, and the banner and help text. |

## Crate Roots

`lib.rs` declares all top-level modules with `pub mod` so the test crates can reach them as `rustr::cli`, `rustr::constants`, and `rustr::helpers`. It contains no implementation.

```rust
// src/lib.rs
pub mod cli;
pub mod constants;
pub mod helpers;
```

`main.rs` is the dispatcher: it parses arguments, applies the self-run check, chooses the project name, and runs one action. It declares the modules it uses with `mod` alongside the `pub mod` declarations in `lib.rs`. The rule for when a project carries a `lib.rs` lives in [`PATTERNS.md`](PATTERNS.md) under "Library + binary split".

## Errors

Fallible functions return `anyhow::Result` and fail through `bail` and `context`.

## Constants

Shared constants live in `src/constants.rs`. The file contains only `pub const` declarations, no functions or logic. Platform-specific values use `#[cfg]` on paired declarations rather than runtime branching.

## Test Organization

Tests live under `tests/` at the project root as top-level test crates, one per source module under test, named `<module>_tests.rs`. Each crate runs on its own with `cargo test --test <name>`.

```
tests/
  cli_tests.rs        # cargo test --test cli_tests
  helpers_tests.rs    # cargo test --test helpers_tests
```

A new test crate is another top-level file in `tests/`, named for the module it exercises.

## File Naming

All source files use `snake_case`. `constants.rs` carries its conventional meaning (`pub const` declarations only). Every other file is named for the concern it implements: `cli.rs` parses arguments, `helpers.rs` does the filesystem and process work. A test crate takes the name of the module it exercises plus `_tests`.

## Directory Layout

```
src/
  lib.rs
  main.rs
  cli.rs
  constants.rs
  helpers.rs
tests/
  cli_tests.rs
  helpers_tests.rs
```
