# Testing Standards

**Test Types**: The suite has two test crates. `tests/cli_tests.rs` exercises argument parsing through `parse_args_from` entirely in memory: option spellings, the `=` and space forms of valued options, the `--` separator, positional project names, and pass-through of unrecognized tokens. `tests/helpers_tests.rs` exercises projects-directory resolution through `projects_dir_from` in memory, and `Cargo.toml` interpretation through `get_binary_name` against temporary project directories that hold a real `Cargo.toml` and, where the case needs it, a real `src/main.rs`.

**Test Runner**: Use `cargo test` as the test runner. All tests live under `tests/` — no in-source `#[cfg(test)]` modules (see [`PATTERNS.md`](PATTERNS.md) "No inline `#[cfg(test)]` modules"). Make functions `pub` and declare their module `pub mod` in `lib.rs` so the test crates can reach them.

**Test Organization**: One test crate per source module, named `<module>_tests.rs` (see [`FILE_CONVENTIONS.md`](FILE_CONVENTIONS.md) "Test Organization"). Name test functions to describe what should happen when specific conditions are met.

**Test Execution**: When working on specific tests, use `cargo test <pattern>` to match test names or `cargo test --test cli_tests` / `cargo test --test helpers_tests` to run one test crate. Focused runs are the iterative inner loop; once they pass, run one full `cargo test` after `cargo fmt` and `cargo clippy --all-targets -- -D warnings` and before considering the Rust work complete, so nothing outside the focused set regressed.

**Test Independence**: Write tests with no dependencies between test functions. Tests must run successfully in isolation or in any order. A test that needs filesystem state creates its own temporary directory with a unique suffix and removes it when done; `TempProject` in `tests/helpers_tests.rs` is the existing shape.

**Cross-Platform Paths**: The project runs on both Linux and Darwin (macOS). On macOS, `/var` is a symlink to `/private/var`, so `std::env::temp_dir()` returns a non-canonical path that differs from what system tools report. Canonicalize a temp root before comparing paths derived from it against anything an external tool prints.

**Consume Production Constructors**: When a fixture needs a value the production code defines, route through it rather than restating it. `tests/helpers_tests.rs` names the manifest file through `rustr::constants::CARGO_TOML` rather than spelling `"Cargo.toml"` a second time. A fixture that re-implements a production value stays internally consistent and keeps passing when production drifts, so the test silently stops exercising the real code path.

**Mocking Strategy**: Test against real implementations. Use temporary directories with real `Cargo.toml` files for binary-name resolution. The suite covers the in-process logic, argument parsing and `Cargo.toml` interpretation. Keep decision logic out of the sites that spawn `cargo` or the built target binary so it stays testable in process.

**Process Exit in the Parser**: `parse_args_from` prints help or the version banner and calls `std::process::exit` when it meets `-h`, `--help`, `-V`, or `--version`, so a test must not pass those tokens to it except after a `--` separator; the test process would exit. Tests that assert on those tokens as pass-through place them after `--`.

**Quality Standards**: Maintain meaningful coverage on the behavior users depend on: every rustr option in both its `=` and space form where it takes a value, the `--` separator, positional project-name selection and its interaction with `--project`, pass-through of unknown tokens including single-dash and triple-dash spellings, projects-directory resolution with the override set, empty, and unset, and binary-name resolution across `default-run`, `[[bin]]` entries, `autobins`, and the ambiguity error. Ensure that both success and failure scenarios are tested.

**Tests Guard Persistent Properties**: A test asserts a property the system holds for as long as it exists, so it is useful on every run for the life of the system. A check whose value lies in one event of that life belongs in a scratch script run during that work: that an edit stayed inside its authorized region, that a value moved as intended, or that a check fails before the fix and passes after it. The script's command and output go in the working record as a receipt, and nothing of it enters `tests/`. The suite carries no test that pins prose the project writes (help text, rendered headings, digests of either) and no test that restates a constant or literal set in code. Such a check is valuable once, when its change lands; kept in the suite, it guards nothing from then on.

**Test Naming**: Use generic, project-neutral names in all test fixtures, examples, and assertions. Do not reference real entities from projects that motivated the test. Use truly anonymous placeholder names like `alpha`, `beta`, `gamma`, `epsilon`, `entity`, `object`, `foo`, `bar` — names that carry no domain connotation and cannot be traced back to any specific project.

**Linting**: Always run `cargo fmt` and `cargo clippy --all-targets -- -D warnings` after writing or modifying tests. Test code is held to the same lint standard as production code.

**When Testing Reveals Issues**: When writing tests that expose bugs or missing functionality in the code under test, stop and report your findings with analysis before making any fixes. Present what you discovered, explain the root cause, and discuss the appropriate solution. Only proceed with fixes after receiving explicit direction, even if the fix seems obvious.
