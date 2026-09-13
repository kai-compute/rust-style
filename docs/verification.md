# Verification Record

Review date: September 14, 2026. Document revision: 1.1.

This record accompanies the [Rust Style README](../README.md). Revision 1.1 follows a technical review of revision 1.0 and preserves its proposed status. Technical verification does not itself adopt this standard for an organization.

## Review Scope

The review distinguishes language and tool behavior, organizational choices, and advice. Rust language facts are checked against primary documentation and selected compiler probes. Rules such as a 100-column formatting target, package naming, a denied lint, and a committed library-workspace lockfile remain explicit project choices.

The review covers Rust 2024, the 1.85.0 compatibility floor, Cargo configuration and test selection, conversion conventions, ownership, panic and I/O contracts, async cancellation, unsafe interfaces, and the distinction between validation and proof of soundness. No language-level correction was needed in the reviewed claims; the executable gate identified one formatting defect.

## Primary-Source Findings

| Claim checked | Finding and primary source |
| --- | --- |
| Edition and MSRV | Rust 1.85.0 stabilized Edition 2024. Parsing edition and formatting style edition are distinct. See the [1.85.0 release announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/). |
| Resolver selection | Edition 2024 implies resolver 3, which defaults to Rust-version-aware fallback. A virtual workspace must select its resolver explicitly. See [resolver migration](https://doc.rust-lang.org/edition-guide/rust-2024/cargo-resolver.html) and [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html). |
| Workspace lint inheritance | Workspace lints require each member's explicit inheritance. This is exercised by the extracted E02 manifests. See [workspace configuration](https://doc.rust-lang.org/cargo/reference/workspaces.html). |
| Lockfiles | Committing a lockfile is consistent with Cargo guidance, but a library's development lockfile does not select every downstream resolution. See [Cargo.toml vs Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html). |
| Cargo test selection | `--all-targets` selects library, binary, test, benchmark, and example target kinds. `--doc` selects library documentation and cannot be combined with other target selections. See [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html). |
| Conversion naming | Conversion prefixes distinguish cost and ownership; `to_` is not an allocation guarantee. The guidelines' `Path::to_str` example returns a borrowed value. See [API naming](https://rust-lang.github.io/api-guidelines/naming.html). |
| Borrowing contracts | `Borrow` requires equivalent equality, ordering, and hashing behavior where those traits are used. See [Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html). |
| Native async trait objects | A native async method is not directly dyn-dispatchable on the tested toolchains. Both compiler probes reject the example with E0038. See [dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility). |
| Edition 2024 unsafe changes | Environment mutation becomes unsafe, extern blocks require `unsafe`, and relevant linking attributes require `unsafe(...)`. See [newly unsafe functions](https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html), [extern blocks](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html), and [unsafe attributes](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html). |
| Unsafe operations in unsafe functions | Edition 2024 warns by default; the document deliberately strengthens this to denial. See [the edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html). |
| Safety and destruction | Unsafe code must prevent safe callers from triggering undefined behavior. Destructors are not guaranteed to run, and struct fields drop in declaration order. See [undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), [mem::forget](https://doc.rust-lang.org/std/mem/fn.forget.html), and [destructors](https://doc.rust-lang.org/reference/destructors.html). |
| Async task and lock behavior | Dropping a Tokio JoinHandle detaches its task. Short synchronous locking can be appropriate in async code when the guard does not span an await. See [JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html) and [shared state](https://tokio.rs/tokio/tutorial/shared-state). |
| Panic handling | `catch_unwind` catches unwinding Rust panics, not aborts, and does not guarantee recovery from arbitrary foreign exceptions. See [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). |

The format, lint severity, naming defaults, review policy, and adoption rules are organizational decisions. Their validity is not inferred merely from links to official Rust documentation.

## Changes From Revision 1.0

- Corrected the B03 test's line wrapping to match the documented Rust 2024 formatter configuration.
- Published the reviewed document as README, retaining its proposed status, rule identifiers, English terminology, and distinction between normative and advisory sections.
- Added reproducible environment setup and validation commands, with the maintained README examples as their source.
- Added an explicit record of verification scope and limitations.

## Automated Checks

Local validation passed on September 14, 2026, on `x86_64-unknown-linux-gnu`, using Nix, devenv 2.3.0, and direnv. CI installs devenv 2.3.1 from the same immutable release revision used for the locked devenv modules.

| Check | Result |
| --- | --- |
| Development toolchain | Rust 1.98.1, commit `48a229cea`, with pinned rustfmt and Clippy; E01 gate passed. |
| Independent MSRV | Rust 1.85.0, commit `4d91de4e4`; all-target compilation and tests passed. |
| Complete examples | Five modules compiled; all six example unit tests passed on each toolchain. |
| Additional boundaries | All seven integration tests passed on each toolchain. |
| Negative compiler probes | All three failed for the intended diagnostic on each toolchain. |
| Documentation | rustdoc completed with warnings denied; separate doctest command completed with zero doctests. |
| Markdown and configuration | Local links, heading fragments, footnotes, whitespace, three TOML blocks, and shell syntax passed. |
| External links | All 60 original reference URLs and four setup URLs returned HTTP 200; the referenced external fragment exists. |
| Environment | `devenv test` passed; `direnv exec . rustc --version` selected the pinned Rust 1.98.1. Nix formatting and ShellCheck passed. |

Run [the repository checks](../README.md#development-environment) to reproduce the current results. Counts describe revision 1.1, not a promise that later edits have been reviewed automatically.

The checks extract five complete Rust modules (B01, B02, B03, B04, B06), the D03 import excerpt, and all three E02 TOML blocks using a Markdown parser. D03 is checked only by rustfmt because it intentionally refers to unspecified surrounding code.

The temporary Cargo workspace uses the exact E02 manifests and a [committed, dependency-free lockfile](../validation/Cargo.lock). The normal gate executes E01's shell commands verbatim. This covers formatting, all-target compilation, Clippy with warnings denied, unit and integration tests, a separate doctest run, and rustdoc with warnings denied. Shell examples also receive a syntax check.

The [boundary tests](../validation/regressions.rs) cover a borrowed lookup surviving its temporary key, exact and missing matches, integer overflow, an oversized frame rejected before writing, an empty frame, partial effects on write failure, and short/interrupted writes without an implicit flush.

Three negative compiler probes check the intended diagnostics for environment mutation without an unsafe block, a non-unsafe extern block in Edition 2024, and a native async method used through `dyn Trait`. The compiler is invoked for these probes; none of their code is executed.

## Limits

- The examples are tested on `x86_64-unknown-linux-gnu`. This is not a cross-platform, `no_std`, or FFI-runtime certification.
- The MSRV checks cover the extracted library and its tests. They do not claim that the Python/Nix tooling runs on Rust 1.85.0 or that every crate adopting the prose is compatible with it.
- The extracted examples contain no runnable rustdoc code blocks. The doctest command is exercised and reports zero doctests; the five Markdown examples are instead compiled as modules and their unit tests executed.
- Reference availability and fragment checks establish that a link resolves, not that every sentence is proven by its target. Semantic review is separate, and rolling upstream documentation may change.
- No Miri, fuzzing, runtime-specific async tests, foreign-library integration, or exhaustive platform/feature matrix is claimed. The supplied examples do not implement those systems.
- No organization has been certified conformant. Technical review does not approve future policy exceptions or replace design and safety review.
