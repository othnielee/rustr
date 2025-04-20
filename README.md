# rustr - Rust/Cargo Task Runner

A simple CLI tool to test, build and run Rust projects with specific command-line arguments.

## Features

- Automatically detects and tests or builds Rust projects
- Supports building in debug and release modes
- Can copy release builds to a specified directory
- Passes through command-line arguments to the target application
- Flexible project selection via current directory, project name, or explicit flag

## Usage

```bash
# Run a project (builds in release mode and executes)
rustr [--project PROJECT] [PROJECT] [ARGS...]

# Run tests for a project
rustr [--project PROJECT] [PROJECT] --test

# Build project
rustr [--project PROJECT] [PROJECT] --build

# Build in release mode
rustr [--project PROJECT] [PROJECT] --release

# Build in release mode and copy to ~/bin (or specified path)
rustr [--project PROJECT] [PROJECT] --release-bin [DESTINATION]
```

### Project Selection

The target project can be specified in three ways:
1. Using the `--project` flag: `rustr --project myproject`
2. As a positional argument: `rustr myproject`
3. Automatically when in a project directory: `rustr`

The `--project` flag takes precedence over other methods.

### Flag Precedence and Behavior

When multiple task runner flags are specified, they are handled in this order:
1. `--test` (highest precedence)
1. `--build`
2. `--release`
3. `--release-bin`

For example:
- `rustr myproject --release-bin --test` will only perform the `--test` operation
- `rustr myproject --build --release --release-bin` will only perform the `--build` operation
- `rustr myproject --release --release-bin` will only perform the `--release` operation
- `rustr myproject --release-bin` will perform the `--release-bin` operation

If no task runner flags are specified, the target project will be built in release mode and executed with any provided arguments.

### Examples

```bash
# Run a project with arguments
rustr myproject --arg1 value1 --arg2 value2

# Build a project in release mode
rustr myproject --release

# Copy release build to ~/bin
rustr myproject --release-bin

# Copy release build to custom directory
rustr myproject --release-bin /path/to/dir

# Run tests for the project in the current directory
rustr --test
```

## Installation

```bash
cargo install --path .
```

## License

MIT