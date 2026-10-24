# Project Patterns

This document captures the cross-project Rust patterns used in every Rust project in this codebase family. These are the structural rules a plan must respect before any feature-level review begins. A plan that violates a pattern here is wrong before its logic is examined.

The patterns are named. Reviewers cite the name when blocking ("violates Thin `mod.rs` rule"). Each entry states the rule, the reasoning where it is load-bearing, and the anti-pattern reviewers flag on.

Companion documents:

- [`AGENTS.md`](../AGENTS.md) — engineering rules (surgical changes, read source first, run `cargo fmt` and `cargo clippy --all-targets -- -D warnings`).
- [`FILE_CONVENTIONS.md`](FILE_CONVENTIONS.md) — file naming, directory layout, where files live.
- [`TESTING.md`](TESTING.md) — test organization, fixtures, mocking strategy, in-process capture.

PATTERNS.md is the architectural layer above those. AGENTS.md says "obey clippy"; PATTERNS.md says which suppressions are specifically forbidden and why. TESTING.md says how tests are organized; PATTERNS.md says tests do not live in `src/`. Patterns are stated as settled rules. Do not reopen them during planning or implementation.

## Module discipline

### Privileged-root rule

Outside the flat-layout-under-budget exception below, only `main.rs`, `lib.rs`, `error.rs`, and at most one thin shared utility file (`util.rs` or equivalent) live as files at `src/` root. Every other concern lives in a module directory.

**Why**: `src/` root is the crate's table of contents. A flat root sprawl trains readers to look everywhere for everything. A privileged root makes the entry points obvious and forces every concern to declare a home.

**Anti-pattern**: dropping a new `foo.rs` at `src/` root for a feature that has its own state, types, or multiple functions. Create `src/foo/` with a `mod.rs` and the feature's files inside.

### Flat layout allowed only when trivial

A project may keep `src/` flat — files at the crate root, no module directories — only while it has roughly 10–15 obvious, single-purpose files. Beyond that threshold, modules are required. The reviewer flags "flat layout creeping past the size budget" as a structural defect on the next plan that adds to a flat root.

**Why**: small CLIs and utilities do not need module ceremony, and adding it early adds friction. But flat layouts that grow past the budget become impossible to navigate, and the cost of restructuring grows with every file added.

**Anti-pattern**: a project at 20+ flat files where the proposed plan adds a 21st without restructuring.

### Thin `mod.rs` rule

A multi-file module's `mod.rs` is a table of contents: submodule declarations and selective `pub use` re-exports. Implementation lives in the sibling files. A `mod.rs` accumulating logic that belongs in a sibling is the anti-pattern. Single-file modules are exempt — a small module's implementation may live in `mod.rs` or in `foo.rs`, but never sprawled across both.

**Why**: a thin `mod.rs` keeps the module's public surface readable in one screen. A fat `mod.rs` hides logic in a file readers reach for to find the module's *shape*, not its *implementation*.

**Anti-pattern**: a `mod.rs` over ~30 lines that contains anything other than `mod`, `pub mod`, and `pub use` statements.

### No inline `#[cfg(test)]` modules

All tests live under `tests/` at the project root. `#[cfg(test)] mod tests { ... }` inside `src/` files is forbidden. To make a function reachable from external tests, make it `pub` and re-export it through the crate's public surface as needed; `pub(crate)` is not visible to the test crate.

**Why**: inline tests entangle test code with production code, encourage testing private internals through visibility leaks, and make the test surface invisible from outside the source tree. An external `tests/` layout keeps production code clean and makes the test suite a first-class artifact.

**Anti-pattern**: `#[cfg(test)] mod tests` anywhere in `src/`. The structural rule overrides any "but this is convenient here" argument.

## Crate shape

### Library + binary split

When anything other than the binary needs the crate's surface — tests, sibling crates, future tooling — the surface is published through `lib.rs`. `main.rs` becomes a thin dispatcher. Binary-private submodules may be declared in `main.rs` itself. A pure single-binary tool with no external consumers may omit `lib.rs`.

**Why**: `lib.rs` makes the testable surface explicit and importable. Putting business logic only in `main.rs` traps it inside the binary and forces every test through subprocess invocation.

**Anti-pattern**: a meaningfully-sized project with all logic in `main.rs` and no `lib.rs`, where tests have no in-process entry point.

### `main.rs` is an orchestrator

`main.rs` invokes CLI parsing through `config/cli` (or the equivalent), calls the config builder, and dispatches to handlers. It does not define clap types and does not implement business logic. Setup, signal handling, and runtime initialization belong in `main.rs`; everything else moves to a module.

**Why**: `main.rs` is the binary entry point, not a feature module. Logic in `main.rs` is invisible to library consumers, hostile to in-process testing, and signals that the dispatcher boundary was not drawn.

**Anti-pattern**: `main.rs` exceeding the dispatch shape — implementing parsing, validation, persistence, or output formatting inline rather than calling into named module functions.

### Workspace splits by responsibility

When a project grows past a single crate, it splits into a Cargo workspace where each crate carves a responsibility (core types, database, CLI, API, orchestrator, agents, …). Crate names follow the `<project>-<role>` pattern. The single-crate patterns in this document apply inside each crate.

**Why**: workspace splits by responsibility scale; splits by file count or convenience produce crates that import each other in circles.

**Anti-pattern**: a workspace where two crates share a responsibility (two crates that both define the same domain types), or a crate split purely to reduce a file count without a coherent responsibility behind it.

## Architectural boundaries

### Dependency direction

Core and domain modules do not import from CLI, output, tracing, or process-execution modules. The reverse direction is normal: CLI imports from core, output imports from core, dispatch imports from core. Dependencies point inward, toward the stable types and rules.

In single-crate projects this applies between modules: `db`, `decision`, `schema`, or whichever modules carry the domain do not pull in `command`, `runner`, `output`, or `cli`. In workspace projects this applies between crates: `<project>-core` does not depend on `<project>-cli`, `<project>-api`, or any leaf crate.

**Why**: a domain module that imports from CLI or output couples its types to a specific delivery channel. The domain becomes harder to test in isolation, harder to reuse across binaries, and harder to evolve when the surface changes. Inward-pointing dependencies keep the domain a stable target everything else builds against.

**Anti-pattern**: a core module that imports clap types to "make it easier to pass arguments through," or a domain function that calls `println!` or a tracing macro instead of returning a typed value.

### Output happens at the dispatch boundary

Domain modules return typed values; rendering — stdout, stderr, tracing, log emission, structured event publishing — happens at the command/dispatch layer. A function that returns a typed value composes; a function that writes to stdout does not.

Where a project has a dedicated emitter abstraction (output module, event publisher), command handlers route through it rather than calling `println!` directly. Where a project is small enough that `println!` in `main.rs` or the command dispatcher is fine, that is also acceptable — but the call sites stay at the dispatch boundary, not buried in domain code.

**Why**: output mixed into domain code is impossible to test in-process, impossible to silence, and impossible to redirect. The dispatch boundary is the one place output decisions belong.

**Anti-pattern**: `println!`, `eprintln!`, or `tracing::info!` calls inside a domain module's logic. Domain modules return values; the dispatcher decides how to render them.

## Configuration and CLI

### `config/` module owns CLI parsing and typed-config construction

The `config/` module is the single home for parsing inputs and producing the typed configuration the rest of the crate consumes. Its standard files are `cli.rs` (clap-derived types), `builder.rs` (the `build_config(&cli) -> Result<Config>` constructor), `model.rs` (the typed `*Config` returned), `constants.rs`, `paths.rs`, `error.rs`, and `types.rs` where shared. `mod.rs` re-exports the public surface.

**Why**: a single config boundary makes every input — CLI flags, environment variables, file-loaded settings — flow through one named place. Downstream code consumes one typed value, not a CLI struct.

**Anti-pattern**: code outside `config/` that reads `std::env::var`, parses additional CLI flags, or constructs configuration from raw inputs; or splitting clap types and config types into separate top-level modules that re-derive each other.

### Typed config does not leak CLI types; domain functions take narrow substructures

After `main.rs` calls `build_config(&cli)`, no other code touches the CLI struct. Command handlers accept the typed `*Config` (or substructures of it). Core and domain functions accept the narrow typed substructure or explicit parameters they actually need — not the whole `*Config`, which carries command variants, output settings, and other dispatch-layer concerns the domain does not own.

**Why**: leaking CLI types couples internal modules to the surface shape. A renamed flag should not force a rename in every handler. Accepting the whole `*Config` in a domain function is a softer version of the same leak: the domain now depends on every field, including those owned by the dispatch layer.

**Anti-pattern**: a handler signature taking `&Cli` or `&CliArgs` instead of `&Config` or a typed substructure; a domain function taking `&Config` when it actually needs a narrow substructure or a small set of explicit parameters.

### One identifiable dispatch surface

Handlers are reached through a single dispatch site, not scattered across the codebase. Three observed shapes are valid:

- **`command/` module** — a dedicated module owns handlers indexed by verb; `main.rs` dispatches on the typed config's command variant into `command::handle_*` entry points. Use this when the project has many verbs and shared handler infrastructure (output emitters, refusal rendering, formatting).
- **Per-verb files at the crate root with a `cli.rs` dispatcher** — each verb lives in its own file (`create.rs`, `list.rs`, `show.rs`); `cli.rs` dispatches into them. This is valid only when the crate is in flat-layout-under-budget territory (per "Flat layout allowed only when trivial"); once the verb count grows past the budget, the per-verb files move into a `command/` module. Use this in workspace-member CLI crates dedicated entirely to the CLI surface.
- **In-main dispatch** — `main.rs` matches on the command variant and calls library functions directly. Use this only in small projects where the handler surface is narrow enough that a dedicated module would be ceremony.

The rule is one dispatch site, not three. A plan that adds a fourth shape or smears handler calls across the codebase violates this pattern.

**Anti-pattern**: a project with handlers reached from multiple unrelated entry points (some via `command/`, some directly from `main.rs`, some via ad-hoc module functions called from unrelated places).

## Build and version surface

### `build.rs` injects version and build metadata

Single-binary projects ship a `build.rs` that injects `APP_NAME`, `APP_VERSION`, `APP_BUILD`, and `APP_COMMIT` into the compiled binary via `cargo:rustc-env`. `APP_NAME` and `APP_VERSION` come from `CARGO_PKG_NAME` and `CARGO_PKG_VERSION`. `APP_BUILD` is a UTC timestamp formatted `%Y%m%d.%H%M`, bucketed down to a 15-minute boundary in release builds to reduce build-time churn; debug builds use full resolution. The bucketing is a churn-reduction measure, not a reproducibility guarantee — true reproducibility would require `SOURCE_DATE_EPOCH`.

`APP_COMMIT` is the output of `git rev-parse --short HEAD`, or `unknown` when git is unavailable or the build runs outside a repository. Cargo reruns `build.rs` only when a watched path changes, so `build.rs` also watches the paths git rewrites when HEAD moves: `HEAD`, `packed-refs`, and the `refs` directory. The directory is watched whole because a commit after `git pack-refs` or `git gc` creates the branch's loose ref file anew under `refs`. Each path is resolved with `git rev-parse --git-path` and emitted only when it exists, because Cargo reruns a build script on every build for a watched path that does not exist. A commit refreshes `APP_COMMIT` and `APP_BUILD` on the next build. `build.rs` runs git through `std::process::Command`.

Binary crates inside a Cargo workspace may follow the same shape with `APP_NAME` taken from the crate name. Library-only crates do not need `build.rs`.

**Why**: every binary needs to know what it is, when it was built, and which commit it was built from. The commit identifies the source a deployed binary was built from, on every machine it runs on. A single canonical `build.rs` shape across projects keeps the version surface uniform and makes the value of `APP_BUILD` predictable for diffing.

**Anti-pattern**: any hardcoded version string in source, a `build.rs` that diverges from the canonical shape, a library-only crate carrying a `build.rs` for binary-style version metadata, or a build dependency such as `vergen` or `built` added to read the commit.

### Startup banner consumes the build metadata

Where a project prints a startup banner, the format is `{APP_NAME} v{APP_VERSION} (build {APP_BUILD}, commit {APP_COMMIT})`, consuming the `env!()` values injected by `build.rs`. Silent-by-default tools (one-shot CLIs whose output is consumed by other tools) may omit the banner; tools with human-facing startup print it in this shape.

**Why**: every banner that already prints version information should print the same shape so operators can read every tool the same way. Divergent banners are an unforced inconsistency.

**Anti-pattern**: a banner that omits any of version, build, or commit, or invents its own format.

## Lint discipline

The clippy `-D warnings` posture is taken seriously: every warning either is fixed at its source or carries a written justification adjacent to its suppression. Suppressions are tiered.

### Function size

Most functions should naturally land under roughly 120 lines. Longer functions are not automatically wrong when they are linear, clearly named, and still express one coherent workflow, but length is design feedback. Reviewers should treat obvious named phases inside a large function as extraction candidates.

Use these ranges as review guidance:

```text
0-80 lines      Normal. Usually not worth discussing on length alone.
80-120 lines    Fine if the shape is linear and the name is precise.
120-160 lines   Review the structure; extract if there are obvious named phases.
160-200 lines   Accept only when the function is still one coherent workflow.
200+ lines      Fails the standing gate; extract rather than suppress.
250+ lines      Clear smell unless it is generated code or a rare table-like case.
```

The preferred range is not the same as the enforced ceiling. The hard policy is that every target must pass `clippy::too_many_lines` at the configured 200-line threshold through `cargo clippy --all-targets -- -D warnings`. A function at 170 lines may still deserve review pressure; a function at 201 lines fails the gate and must be split.

### Forbidden suppressions

The following `#[allow(...)]` attributes are forbidden. Each one bypasses a real fix. Instances in the tree are technical debt to retire, not precedent to follow.

- `dead_code` — delete the item, or make it live with a real caller or test.
- `unused_imports` — delete the import.
- `unused_variables` — prefix with `_` at trait or signature boundaries where the parameter is structurally required; otherwise delete.
- `unused_mut` — remove the `mut`.
- `unused_assignments` — investigate; almost always a write-without-read defect.
- `unreachable_code` — fix the control flow.
- `deprecated` — migrate off the deprecated API. Suppression accumulates upgrade debt.
- `clippy::too_many_arguments` — group related parameters into a cohesive `*Params`, `*Context`, or `*Options` struct passed by reference, and update every call site.
- `clippy::too_many_lines` — extract functions until the lint passes on the original.
- `clippy::cognitive_complexity` — break the function up.

**Why**: every entry above is a lint where suppression is functionally equivalent to "I decided not to do the fix." The fixes are mechanical and the lint is the design feedback.

**Anti-pattern**: any new `#[allow(...)]` naming one of the lints above. Reviewers block on sight.

### `clippy::correctness` suppressions are defects

The `clippy::correctness` lint group flags near-certain bugs. Suppressing any lint in this group requires an adjacent `// reason: ...` comment naming the specific invariant that makes the lint a false positive at this site, and is treated as a defect to revisit. A bare `#[allow(clippy::correctness::...)]` is rejected the same way as the forbidden suppressions above.

**Why**: clippy flagged a probable bug. Someone silencing it without naming why is the most expensive form of suppression debt — the warning was load-bearing and now the reason it was load-bearing is invisible.

**Anti-pattern**: a `clippy::correctness` allow with no comment, or with a comment that does not name the invariant.

### All other suppressions require a written reason

Every other `#[allow(...)]` — including legitimately suppressible lints like `clippy::large_enum_variant`, `clippy::module_inception`, `clippy::needless_pass_by_value`, `clippy::result_large_err`, the `clippy::cast_*` family, and anything in `clippy::pedantic`, `clippy::nursery`, or `clippy::restriction` — requires an adjacent `// reason: ...` comment naming why the lint is a false positive in this site. A bare `#[allow(...)]` is rejected.

**Why**: the cost of a suppression is paid by the next reader, not the one writing it. A written reason makes the trade-off auditable and lets future cleanups distinguish "still a false positive" from "no longer applies."

**Anti-pattern**: a bare `#[allow(...)]` attribute anywhere in the tree, regardless of which lint it names.

## Tooling and dependencies

### `Cargo.lock` is committed

Every binary crate commits `Cargo.lock`. This is standard Rust practice for binary crates and is consistent across every project in the family. Library-only crates may omit it; workspace roots commit it.

**Anti-pattern**: `.gitignore`ing `Cargo.lock` in a binary crate.

## Where the rest lives

This document covers cross-project Rust patterns — the structural rules a plan is checked against before feature-level review. Other concerns live elsewhere:

- **Engineering rules** — [`AGENTS.md`](../AGENTS.md). Surgical changes, read source first, run `cargo fmt` and `cargo clippy --all-targets -- -D warnings`, prefer stable patterns, meaningful names.
- **Module layout, file naming, directory structure** — [`FILE_CONVENTIONS.md`](FILE_CONVENTIONS.md). Multi-file modules use `foo/mod.rs`. Tests are top-level crates under `tests/`.
- **Test organization and standards** — [`TESTING.md`](TESTING.md). Real implementations over mocks. Temp directories with real files. Canonicalize paths on macOS. Generic placeholder names in fixtures.

A proposed change that respects the module discipline, the crate shape, the architectural boundaries, the config and CLI rules, the build and version surface, the lint posture, and the tooling posture is structurally aligned. Logic correctness, feature shape, and syntax cleanliness are reviewed separately on top of that baseline.
