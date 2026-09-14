# Verification Report

Review date: September 14, 2026. Document revision: 1.1.

The reviewed [FUNCTIONAL.md](../FUNCTIONAL.md) preserves the supplied document's 15-chapter structure and all 25 independent Rust examples. This is an engineering review with executable evidence, not formal certification of every normative statement.

## Changes from the supplied document

- Replaced the uncompiled status with the measured compiler and execution matrix below.
- Formatted 15 listings using the pinned Rust 2024 rustfmt configuration.
- Added `Display` and `std::error::Error` implementations to Example 25's record and pipeline errors. Error messages now expose retained fields, and `source()` preserves underlying parse and I/O causes. No warning suppression was added.
- Replaced the inline, unexecuted Python checker with the maintained [Markdown parser and Rust runner](../scripts/verify_functional.py). Tests compile against the FUNCTIONAL.md listings themselves.
- Added 19 tests for laws, ownership, parser boundaries, asynchronous sequencing, and I/O failures.
- Added locked [devenv](../devenv.nix), [direnv](../.envrc), and [GitHub Actions](../.github/workflows/ci.yml) configuration.
- Replaced the unsupported "supplied PDF" citation and printed page ranges with public publisher and chapter links. The input for this review was Markdown only.

## Compiler and execution evidence

Target: `x86_64-unknown-linux-gnu`. Edition: Rust 2024. All examples use only the standard library.

| Toolchain | Purpose | Profiles | Example executions | Additional test executions |
| --- | --- | --- | --- | --- |
| Rust 1.85.0 | Independent MSRV gate | Debug and release | 50 | 38 |
| Rust 1.98.1 | Pinned development toolchain | Debug and release | 50 | 38 |
| Total | | Four combinations | 100 | 76 |

Debug means `-C opt-level=0 -C overflow-checks=yes`; release means `-C opt-level=3 -C overflow-checks=no`. Both use `--edition=2024 --forbid=unsafe_code --deny=warnings`. `rustc --test` builds the additional test modules with the same flags. Every example's `main` assertions also execute in release mode.

The environment locks devenv modules to v2.3.1 and records exact nixpkgs and Rust overlay revisions in [devenv.lock](../devenv.lock). The local verification used devenv CLI 2.3.0; CI installs the CLI from the locked v2.3.1 revision. Toolchain versions are checked before compilation. The MSRV executable is selected independently of the development compiler.

## Additional tests

| Listing | Tests | Coverage |
| --- | --- | --- |
| [03: Persistent stack](../validation/functional/example03.rs) | 2 | Shared non-`Clone` payload lifetime; iterative destruction of a 100,000-node unique chain. |
| [15: Parser combinators](../validation/functional/example15.rs) | 3 | Successful choice suppresses alternatives; farther diagnostics retain commitment; UTF-8 byte offsets and repetition progress. |
| [16: Monoids](../validation/functional/example16.rs) | 2 | Ordered partition laws; noncommutativity; counterexamples for checked signed and floating-point addition. |
| [19: Validation](../validation/functional/example19.rs) | 3 | Identity and associativity over success/error combinations; ordered accumulation and empty traversal; first-error changes under phase fusion. |
| [22: Futures](../validation/functional/example22.rs) | 2 | Pending stages preserve continuation order and call count; cancellation drops the owned input. |
| [25: Streaming](../validation/functional/example25.rs) | 7 | Empty input, chunk sizes 1 through 16, UTF-8 classification, LF/CRLF/unterminated records, exact size limits, grammar and numeric limits, reader failure and release, partial writes, final flush failure. |

## Technical source review

The review checked the document's core Rust claims against primary documentation, separately from HTTP link checks:

| Subject | Primary source and reviewed contract |
| --- | --- |
| Edition baseline | [Rust 1.85.0 announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/): Rust 2024 stabilized in this release. |
| Closures and ownership | [Closure types](https://doc.rust-lang.org/reference/types/closure.html#call-traits-and-coercions): closure use determines callable traits; `move` capture does not imply `FnOnce` only. These traits do not establish purity. |
| Evaluation order | [Expressions](https://doc.rust-lang.org/reference/expressions.html): the specified operands evaluate left to right. The document distinguishes that order from algebraic equivalence. |
| Persistence | [`Rc::try_unwrap`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap): extraction requires a unique strong owner. Shared payloads still need a stable value contract. |
| Iteration | [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html): short-circuiting, consumption, and exhaustion guarantees depend on the specific interface. |
| Arithmetic | [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html) and [operator expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html): explicit modular and checked operations are distinguished from ordinary overflow checks. Finite counterexamples also test invalid reassociation claims. |
| Scoped threads | [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) and [`ScopedJoinHandle`](https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html): scopes join outstanding job bodies; explicit joins expose worker panics as results. |
| Asynchrony | [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html): pending work requires a wake protocol; polling after completion has no general useful guarantee. The FUNCTIONAL.md's ready-only fixture is not an executor. |
| Destruction | [`mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html): Rust does not guarantee that all destructors execute. RAII does not replace an external completion protocol. |
| Streaming | [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html), [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html), and [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html): bounded records, partial progress, short writes, explicit flushing, and errors ignored during buffered-writer destruction. |

Manning's [publisher page](https://www.manning.com/books/functional-programming-in-scala) confirms the authors, September 2014 publication, first-edition ISBN 9781617290657, and 320-page extent. Its [public liveBook introduction](https://livebook.manning.com/book/functional-programming-in-scala) confirms the four-part organization. All 15 public chapter pages were checked for chapter titles. These public excerpts do not provide the complete book; printed page ranges and the full detail of each conceptual attribution were not independently verified.

## Repository Integration

The document is now maintained as [FUNCTIONAL.md](../FUNCTIONAL.md) in `kai-compute/rust-style`. The original publication was commit `537f40dc0069ab2dd11001347b7392eef1c902ce` in the former standalone repository. Its examples and six test modules were retained, with paths updated for the shared environment. The shared verification entry point runs both documents on the development toolchain and MSRV, and checks their combined references. Counts above describe the original functional document verification.

## Reproduction

Before integration, local verification passed for both compiler gates, all 72 external URLs, and 45 local links. Actionlint accepted the GitHub workflow. The direnv integration was exercised by authorizing `.envrc` and running the complete MSRV gate through `direnv exec . verify-msrv`.

Install [Nix and devenv](https://devenv.sh/getting-started/) and configure the [direnv hook](https://direnv.net/docs/hook.html). Then:

```sh
direnv allow
verify
verify-msrv
verify-links
```

Without direnv integration, `devenv test` runs both code gates and `devenv shell -- verify-links` checks external references. The code gates validate Markdown whitespace, footnote resolution, local links and anchors, Rust example numbering, and Bash syntax. The development gate also checks rustfmt, Nix formatting, and ShellCheck. External checks follow URLs and inspect requested HTML fragment targets; successful HTTP responses alone are not factual verification.

## Limits

This matrix covers one Linux target and two specific Rust releases. It does not certify Windows, macOS, other architectures, later compilers, every feature shown in current Rust documentation, or downstream adaptations. The sample assertions and 19 extra tests are finite evidence, not exhaustive proofs of laws, purity, termination, or arbitrary callback behavior.

The async tests manually poll controlled futures, including pending states, with a no-op waker. They validate sequencing and ownership under that explicit polling schedule, not readiness notification, runtime integration, scheduling fairness, or cancellation of detached work. Streaming fault injection covers the listed I/O contracts, not every OS or device failure. No Clippy, Miri, performance benchmark, or formal proof is claimed.
