# Repository Guidelines

This document provides guidelines for AI agents working on this Rust codebase. It covers architectural principles, coding standards, and development workflows to ensure consistent, maintainable code.

### Coding Rules

1. **Stay focused on the specific issue being discussed.** Do not provide feedback on unrelated code, features, or theoretical concerns that aren't directly relevant to the issue at hand.

2. **Avoid perfectionism and bikeshedding on theoretical issues.** Recognize when the current implementation is good enough for its intended purpose rather than pursuing theoretical perfection.

3. **When editing code, make surgical changes directly related to the feature or issue.** Do not rewrite blocks of code for stylistic reasons. Only adjust code when there is a concrete functional gap to close - no stylistic or "nice to have" refactors.

4. **Do not jump straight into writing code when starting a new task.** First analyze the project, read the relevant source code, understand the requirements thoroughly, and discuss the scope of work before beginning implementation. Only start coding immediately when given a straightforward, obvious command to execute specific work.

5. **Actually read source files using available tools before providing any analysis or starting work.** Do not rely on memory, assumptions, or patterns you think might exist without examining the actual implementation.

6. **Prefer stable, robust, and idiomatic patterns over clever solutions.** Choose boring but reliable approaches that are easy to understand and maintain, even if they require a bit more code.

7. **Build solutions appropriate for our actual scale while maintaining robustness.** This system is a tool for a single user operating in a small team, so focus on genuine improvements that matter at this scale rather than pursuing best practices designed for enterprise systems or theoretical perfect solutions. Favor efficiency and performant code, but do not consider solutions for concurrent users, thousands of requests per second or hundreds of services.

8. **Prioritize readability and maintainability as primary concerns.** Write code that future developers can easily understand and modify, even if it means being more verbose.

9. **Use meaningful, human-readable names throughout your code.** Avoid terse or obscure abbreviations like "hasPk", "idx", "ctx", "evt", "cfg", "op", "it", "arg", "out" in favor of descriptive names that clearly express purpose and intent.

10. **Keep commenting minimal and purposeful.** Do not add extensive comments or verbose documentation unless explicitly requested. When adding comments, use short, meaningful statements that explain non-obvious implementation details or novel approaches where the code cannot be self-documenting. Never narrate: a comment describes the code as it stands — never the change you just made, what the code "now" does, or what it replaced. The diff is not the audience. If deleting the comment loses nothing the code doesn't already say, delete it.

11. **Communicate clearly in prose using plain language.** Write in complete sentences using terms that any developer can understand, avoiding jargon and preferring natural sentences and paragraphs over bullet points or tables.

12. **Always run `cargo fmt` and `cargo clippy --all-targets -- -D warnings` before considering any Rust work complete.** Catch compiler errors and type issues as part of your standard workflow to ensure code quality. Use focused `cargo test <pattern>` (or `cargo test --test cli_tests` / `cargo test --test helpers_tests`) as the iterative inner loop while implementing, then run one full `cargo test` after the focused tests pass and before considering the Rust work complete.

## Task Runner Contract

rustr exists to test, build, run, and install *other* Rust projects, found in the current directory or by name under the projects directory, which the `RUSTR_PROJECTS_DIR` environment variable sets and which otherwise defaults to `RUST_PROJECTS_DIR` under the home directory. It refuses to operate on itself; inside this repository use `cargo build`, `cargo test`, and `cargo clippy` directly.

The option set is deliberately small: `--test`, `--build`, `--release`, `--release-bin [DESTINATION]`, `--project NAME`, `-h`, `-V`, and the `--` separator. Every token the parser does not recognize is forwarded to the target binary unchanged, and the first bare token becomes the project name when `--project` is absent. That pass-through is the product: `rustr myproject --verbose --port 8080` must deliver exactly `--verbose --port 8080` to the target. The parser is hand-written so that unrecognized tokens are forwarded as data, and every change to it must preserve that. Adding an option to rustr removes that spelling from what a target can receive, so add one only when the task runner itself needs it, support both the `--name value` and `--name=value` forms where it takes a value, and keep `--` working as the escape hatch.

## Computed Claims

This rule applies to all work: code, reviews, drafts, documentation, and analysis. If a statement involves a count, an enumeration, a sweep, a comparison, or any "all/every/none/only" claim, compute it with a scratch script (bash or python) — never assert it from reading, memory, or a prior record. Record the command and its output beside the claim, so a reviewer can re-run the receipt instead of re-deriving the fact.

A receipt whose informative result can be empty must print the size of the set it searched, in the same block — "N examined, M matched" — so a false clean is distinguishable from a true one. Derive that set from the tool (a diff, a file list, a query) rather than typing paths or symbol names into the command; a typo must fail loudly rather than return zero. When the claim quantifies over a set, the denominator is that set, not the number of matches.

## Code Review

When reviewing code, focus on correctness and maintainability rather than formatting nits. If everything holds up, say so plainly. State each finding in decisive, directive language with no hedging, giving its reason, the exact file and line, and a prescribed change. Separate required changes (defects you can already see are wrong, which must be closed before shipping) from open risks (genuinely speculative concerns that only surface at runtime, which the reviewer records rather than accepts and which stay open until later evidence or direction closes them). A concern you can already see is a required change, not an open risk. End with a clear verdict.

## Additional Guidelines

For project-wide architectural patterns, see [docs/PATTERNS.md](docs/PATTERNS.md).

When creating modules, adding mod.rs files, or reorganizing the module tree, read [docs/FILE_CONVENTIONS.md](docs/FILE_CONVENTIONS.md) before proceeding.

For testing standards and practices, see [docs/TESTING.md](docs/TESTING.md).

For how to write comments, records, reviews, and chat replies, see [docs/COMMUNICATION.md](docs/COMMUNICATION.md).
