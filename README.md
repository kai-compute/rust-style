# Rust Style

[Overview](#overview) · [Style Guide](#style-guide) · [Style Decisions](#style-decisions) · [Best Practices](#best-practices) · [Enforcement](#enforcement) · [Repository Maintenance](#repository-maintenance) · [References](#references) · [Rust Libraries](LIBRARIES.md)

The companion [Functional Programming in Rust](FUNCTIONAL.md) covers functional design, composition, state, and effects through 25 executable examples.

The [Rust Libraries](LIBRARIES.md) catalog is a maintained starting point for choosing established dependencies and reviewing their supply-chain risks.

## Overview

### About

This document defines a consistent style for Rust code that is readable, reliable, reviewable, and maintainable. It covers source presentation, API design, ownership, error handling, concurrency, unsafe code, testing, and the build practices necessary to keep those conventions enforceable.

Its organization follows the separation of foundational guidance, specific decisions, and advisory practices used by Google's Go Style documents. The Rust rules, examples, and adoption policy below are independently written. This is not a Google publication, an official Rust Project standard, or a claim that every Rust project should make the same engineering choices.[^google-overview]

The intended readers are authors, reviewers, maintainers, and tool owners. Authors should understand the Style Guide and consult the relevant Style Decisions while working. Reviewers should distinguish a requirement from a recommendation and identify the rule behind a requested change. Tool owners should implement the enforceable requirements without inventing additional policy through configuration.

Style serves engineering outcomes. Compliance does not establish functional correctness, memory safety, security, or suitability for a safety-critical application. Conversely, code that compiles and passes its tests is not necessarily well designed or conformant.

### Document structure

| Part | Purpose | Normative | Canonical |
| --- | --- | --- | --- |
| Overview | Defines scope, terminology, authority, and adoption. | Yes, for those definitions. | Yes. |
| Style Guide | Establishes durable principles and essential requirements. | Yes. | Yes. |
| Style Decisions | Resolves recurring implementation and review questions. | Yes. | No. |
| Best Practices | Presents useful approaches and their trade-offs. | No. | No. |
| Enforcement | Defines conformance checks, configuration patterns, and exceptions. | Yes, except explicitly illustrative examples. | No. |
| References | Identifies primary sources for language and tooling facts. | No. | No. |

This distinction is intentional: the Guide determines the standard; Decisions apply it; Best Practices do not create additional merge requirements. The organization is adapted from the referenced Go documents, not their language-specific prescriptions.[^google-guide][^google-decisions][^google-practices]

### Scope and baseline

The standard applies to first-party Rust libraries, applications, tests, examples, benchmarks, procedural macros, and build scripts. Vendored dependencies and generated artifacts have the special treatment described in [D24](#d24-generated-code-and-third-party-boundaries). Documentation examples are part of the maintained code surface, not an exemption from quality requirements.

The baseline is **stable Rust with an explicitly declared edition and minimum supported Rust version**. New projects adopting this revision use Edition 2024 unless a documented compatibility requirement calls for an earlier edition. Edition 2024 first became available in Rust 1.85.0. The example manifests therefore use `rust-version = "1.85"`; this is an illustrative compatibility floor, not a recommendation to deploy an old toolchain.[^rust-185]

A repository chooses an approved compiler release, supported targets, dependency policy, and minimum supported Rust version. This document does not assume that every crate is asynchronous, portable to every platform, compatible with `no_std`, or intended for publication. Requirements specific to those capabilities apply when the capability is implemented or promised.

Rust snippets are examples of the local rule they accompany, not complete application templates. Blocks explicitly labeled **excerpt** depend on surrounding project code. All other Rust blocks are intended to be self-contained module-level examples; they need not define `main`. Configuration examples are illustrative and do not select an organization's production toolchain or release policy.

### Definitions

**Canonical** means foundational and intended to change infrequently. **Normative** means used to determine conformance. A detailed decision can be normative without being canonical. **Idiomatic** means consistent with recognizable Rust conventions; it does not mean shortest, most generic, or most frequently repeated online.

**MUST** and **MUST NOT** express requirements. **SHOULD** and **SHOULD NOT** express a strong default that may be departed from when a specific engineering reason is documented in the change or nearby design documentation. **MAY** identifies a permitted choice. These capitalized terms have this meaning in the normative parts of this document; ordinary descriptive prose and examples do not silently introduce additional requirements.

A **package** is a Cargo distribution and manifest unit; a **crate** is a Rust compilation unit. A package can contain multiple crates. A **workspace** groups packages under shared Cargo configuration. These terms MUST NOT be used interchangeably in build or API documentation.[^cargo-manifest][^cargo-workspaces]

A **public API** is any interface supported for use outside its implementation boundary. It includes reachable public Rust items, exported macros, supported feature flags, externally consumed formats, and documented runtime behavior. An internally published crate still has a public API relative to its consumers.

A **safety invariant** is a condition whose violation can cause undefined behavior. A **domain invariant** is a condition required for the abstraction's intended meaning. A domain invariant does not become a caller safety obligation merely because an implementation uses unsafe code internally.

The **minimum supported Rust version**, or **MSRV**, is the oldest Rust toolchain the package promises to support under its documented support policy. The language edition, formatting style edition, MSRV, and exact development toolchain are separate settings and MUST be treated separately.[^cargo-msrv][^rustfmt-config]

### Authority and conflicts

Language semantics, platform contracts, and safety obligations take precedence over every stylistic preference. This document cannot authorize undefined behavior or change the contract of a standard-library function. The Rust Reference and the documentation for the actual supported toolchain determine language and API behavior; the Rust Style Guide and `rustfmt` establish the upstream formatting baseline.[^rust-style][^rust-ub]

Within this standard, the Overview's definitions and the Style Guide take precedence over Style Decisions. Decisions take precedence over undocumented local habits. Best Practices are advisory. Repository-specific policy may fill gaps or strengthen a default but MUST NOT silently contradict a requirement. An apparent contradiction is a documentation defect to resolve, not an opportunity to select the easier rule.

The pinned formatter's output is authoritative for syntax layout within its supported scope. A reviewer MUST NOT request a hand-formatted alternative that the approved formatter would reverse. Language facts in the references are not blanket endorsements of every organizational recommendation in this document.

### Adoption and change

New code and materially changed code MUST conform. Adoption does not require unrelated rewrites of an existing repository. Maintainers MUST identify the scope of legacy exceptions and a migration policy; otherwise, “existing style” becomes an unbounded exemption.

A style-only change SHOULD be separate from a behavioral change when separation improves review. Authors MUST NOT introduce an incompatible API change, alter a wire format, or reorder semantically significant operations merely to satisfy a cosmetic preference. Safety defects are not grandfathered by a style migration plan.

### Navigation

| Area | Decisions |
| --- | --- |
| Presentation and organization | [D01 Formatting](#d01-formatting), [D02 Naming](#d02-naming), [D03 Modules and imports](#d03-modules-and-imports), [D04 Visibility and item organization](#d04-visibility-and-item-organization) |
| Expressions and data | [D05 Bindings and control flow](#d05-bindings-and-control-flow), [D06 Types, numbers, text, and collections](#d06-types-numbers-text-and-collections), [D07 Ownership and borrowing](#d07-ownership-and-borrowing) |
| Interfaces and behavior | [D08 Construction](#d08-construction-and-validity), [D09 Traits and generics](#d09-traits-generics-and-conversions), [D10 Errors and panics](#d10-errors-and-panics), [D11 Documentation](#d11-documentation-and-comments), [D12 Resources and I/O](#d12-resources-and-io) |
| Advanced implementation | [D13 Concurrency](#d13-concurrency-and-shared-state), [D14 Async](#d14-asynchronous-code), [D15 Unsafe](#d15-unsafe-code), [D16 FFI](#d16-foreign-function-interfaces), [D17 Macros and attributes](#d17-macros-attributes-and-conditional-compilation) |
| Delivery and maintenance | [D18 Toolchains and Cargo](#d18-toolchains-and-cargo-configuration), [D19 Dependencies and features](#d19-dependencies-lockfiles-and-features), [D20 Tests](#d20-tests-and-examples), [D21 Performance](#d21-performance), [D22 Portability](#d22-portability-and-no_std), [D23 Compatibility](#d23-api-evolution-and-compatibility), [D24 Generated code](#d24-generated-code-and-third-party-boundaries), [D25 Diagnostics and sensitive data](#d25-diagnostics-and-sensitive-data) |

## Style Guide

This part is **normative and canonical**. Rule identifiers are stable review references, not severity levels.

### G01. Correctness and safety precede style

Code MUST preserve the language, resource, and domain invariants on which its behavior depends. A shorter expression, a lint suggestion, or consistency with neighboring code is not a justification for weakening a check or obscuring a safety argument.

Safe Rust SHOULD be the default implementation language within a Rust project. Unsafe code MUST have a necessary, reviewable purpose and a sound interface. An unsafe block transfers proof obligations to the author; it does not suspend Rust's requirements.[^rust-ub]

A public safe function MUST remain memory-safe for all inputs that can be supplied by safe Rust. It may reject input or panic according to its contract, but it MUST NOT rely on an unchecked, documentation-only promise from a safe caller to avoid undefined behavior.

### G02. Optimize for the reader

Code SHOULD make its intent, data flow, ownership, effects, and failure behavior evident at the point where each matters. The relevant reader is a competent Rust programmer who understands the domain at the level required by the module, not the original author with the entire implementation in mind.

Names SHOULD identify domain concepts rather than incidental storage choices. Comments SHOULD explain constraints, decisions, and non-obvious consequences. An abstraction SHOULD remove a concept the reader no longer needs, rather than move essential information to another file.

### G03. Use the least sufficient mechanism

An implementation SHOULD use the simplest mechanism that meets its actual correctness, performance, and extensibility requirements. A function is preferable to a trait when no meaningful polymorphic contract exists; a concrete type is preferable to a web of parameters when those parameters do not represent supported variation.

Complexity MAY be justified by a measured performance need, a genuine interoperability requirement, or a clearer public contract. The justification SHOULD be recorded where a future maintainer can find it. Speculative extensibility is not, by itself, such a justification.

### G04. Make ownership and effects intentional

An API MUST request only the ownership and mutability its contract needs. Retaining data, sharing state, blocking a thread, spawning work, allocating substantially, or performing I/O MUST NOT be hidden behind a misleading name or an apparently harmless accessor.

Resource lifetimes SHOULD follow explicit ownership and lexical scopes. Shared ownership, interior mutability, and global state SHOULD be introduced to model a requirement, not merely to silence the borrow checker.

### G05. Make failure actionable

Expected operational failure MUST be represented in the API rather than converted into an undocumented panic or silently discarded. Errors SHOULD retain enough typed information and contextual detail for the caller to choose an appropriate response.

Panic behavior, partial effects, cleanup limitations, and cancellation behavior MUST be documented when they are relevant to using an interface correctly. A test passing on the success path is not a substitute for a defined failure contract.

### G06. Keep interfaces small and durable

Visibility, trait bounds, dependencies exposed in signatures, and supported configuration combinations MUST be deliberate. Public API is a commitment, not a convenience for reaching implementation details from tests.

Types SHOULD express meaningful distinctions and preserve their own invariants. Interfaces SHOULD make ordinary correct use straightforward without forcing every caller to understand implementation details. Generality that materially complicates common use requires a concrete beneficiary.

### G07. Delegate mechanical presentation

First-party Rust source MUST conform to the approved `rustfmt` output and Rust naming conventions as specified in the Decisions. Authors and reviewers MUST NOT spend review effort on choices already determined by the formatter.

Formatting, lint configuration, and toolchain changes SHOULD be reviewed as policy changes when they alter the effective standard. A tool's newly available option does not automatically become a project requirement.

### G08. Maintain what the project promises

Every claimed edition, MSRV, target, feature configuration, and public behavior MUST have an appropriate validation strategy. Builds and tests MUST NOT rely on accidental properties of a developer's machine.

An implementation SHOULD include tests and documentation proportionate to its failure modes and maintenance risk. A smaller untested implementation is not inherently simpler to maintain than a slightly larger implementation with explicit contracts.

### G09. Be consistent without preserving mistakes

For matters left open by this document, authors SHOULD follow the nearest coherent convention: first the abstraction, then the module or crate, then the workspace. The same concept SHOULD have the same name and behavior across related interfaces.

Reviewers MUST distinguish a defect, a requirement violation, a justified recommendation, and a personal preference. A preference alone MUST NOT block a conforming change. Local consistency MUST NOT perpetuate unsoundness, conceal failure, or expand a known deviation.

## Style Decisions

This part is **normative but not canonical**. Each decision applies the Guide to a recurring engineering choice. A requirement applies only within its stated scope.

### D01. Formatting

Rust source MUST use UTF-8, spaces rather than indentation tabs, four-space indentation, and the approved formatter configuration. Files MUST end with a newline and MUST NOT contain trailing whitespace. Source line endings MUST be normalized consistently; this standard uses LF.

The formatting width is **100 columns as a formatting target**, not a requirement to split every literal or token sequence. Long URLs, strings, generated tokens, and constructs that `rustfmt` cannot usefully wrap MAY exceed the target. Authors SHOULD restructure an excessively dense expression before adding layout workarounds. These choices follow the upstream formatting baseline while making the repository's tool settings explicit.[^rust-style][^rustfmt-config]

Authors MUST let `rustfmt` determine indentation, brace placement, trailing commas, and expression wrapping. Manual column alignment and discretionary formatting changes that will not survive the formatter MUST NOT be introduced. A string's contents MUST NOT be changed solely to satisfy a line-width preference.

`#[rustfmt::skip]` MAY be used for a narrow construct whose layout carries meaningful information, such as a carefully organized data table, or for a documented formatter limitation. Each use MUST have an adjacent explanation. Whole-module or whole-file skips require an approved exception. A skip does not waive naming, correctness, documentation, or review requirements.

Only stable configuration options supported by the approved formatter SHOULD be used. Formatting unexpanded macro input that the formatter does not understand remains an author responsibility.

### D02. Naming

Identifiers MUST follow the casing in this table. Acronyms in type names are treated as words: `HttpClient`, `UserId`, and `Uuid`, not `HTTPClient`, `UserID`, and `UUID`.[^rust-naming]

| Entity | Form | Example |
| --- | --- | --- |
| Modules and source file stems | `snake_case` | `frame_reader`, `frame_reader.rs` |
| Functions, methods, fields, bindings | `snake_case` | `read_frame`, `retry_limit` |
| Structs, enums, traits, type aliases | `UpperCamelCase` | `FrameReader`, `ReadError` |
| Enum variants | `UpperCamelCase` | `WouldBlock`, `EndOfStream` |
| Constants, statics, const parameters | `SCREAMING_SNAKE_CASE` | `MAX_FRAME_LEN`, `N` |
| Type parameters | Concise `UpperCamelCase` | `T`, `Reader`, `Item` |
| Lifetimes | Short lowercase names | `'a`, `'input`, `'de` |
| Function-like macros | `snake_case!` | `frame_set!` |

Names MUST distinguish concepts that differ in meaning, units, ownership, or role. Unexplained abbreviations, type encodings such as `str_name`, and vague names such as `data2` SHOULD NOT be used. Short conventional names are appropriate in a small scope where their meaning is immediately evident.

For new packages, this standard chooses lowercase, hyphen-separated Cargo package names, such as `frame-codec`, with the corresponding underscore-separated crate identifier, `frame_codec`. This is a project naming decision, not a claim that Cargo prohibits underscores. Existing published names MUST NOT be changed for cosmetic conformity.[^cargo-manifest]

Predicates SHOULD read as predicates: `is_empty`, `has_pending`, or `can_retry`. A plain accessor SHOULD use the property name, such as `timeout`, rather than `get_timeout`. Names such as `get` remain appropriate for established lookup APIs. `_mut`, `iter`, `iter_mut`, and `into_iter` MUST reflect their conventional borrowing or consuming roles.[^api-naming]

A function that performs significant work SHOULD use an informative verb such as `parse`, `load`, `fetch`, `encode`, or `compute`. Suffixes such as `_impl`, `_internal`, or numbered variants SHOULD NOT substitute for a meaningful distinction. A raw identifier such as `r#type` MAY be used for interoperability or compatibility, not to make new APIs unnecessarily awkward.

### D03. Modules and imports

A module SHOULD represent a coherent responsibility or abstraction boundary. A crate SHOULD have a stated purpose. Generic dumping grounds such as `utils`, `common`, and `misc` SHOULD be replaced by names describing the responsibility they actually contain.

For new module trees, use `name.rs` with a `name/` directory for child modules when needed. Existing `name/mod.rs` layouts MAY remain. A layout migration SHOULD NOT be mixed with unrelated changes, and a project MUST NOT maintain two competing source files for the same module.

Imports MUST make name provenance reasonably clear. Group them in this order, separated by a blank line: `core`, `alloc`, and `std`; external crates; the current crate and relative module paths. Let the formatter sort within groups, including its special treatment of `self` and `super`. Do not require unstable import-grouping options to implement this policy.[^rust-style-items]

**Excerpt — import organization:**

```rust
use std::collections::BTreeMap;
use std::io::{self, Read};

use example_transport::Connection;

use super::ReadError;
use crate::frame::Frame;
```

Glob imports MUST NOT be used in production code except for an explicitly documented prelude or a narrowly scoped enum-variant import that improves a match. `use super::*;` MAY be used inside a unit-test module. Public glob re-exports require a deliberate API decision and MUST NOT accidentally expose a dependency's evolving namespace.

Imports SHOULD appear at module scope. A local import MAY be used to limit a trait's scope, resolve a collision, or accompany a genuinely local item. Prefer a qualified name or an informative alias when importing a common name would obscure meaning. Importing a trait as `_` MAY be used when only its methods are needed and naming it would create a conflict.

Modern-edition code MUST NOT add redundant `extern crate` declarations. Cases such as explicitly making `alloc` available in a `no_std` crate or an intentional crate alias are not redundant and MAY be used when needed.

### D04. Visibility and item organization

Items MUST be private unless another module or crate needs the capability they expose. Use the narrowest visibility that expresses the intended boundary: private, `pub(super)`, an appropriate `pub(in ...)`, `pub(crate)`, or `pub`.

Tests MUST NOT force an implementation detail into the public API. Prefer colocated unit tests for private behavior and integration tests for the supported external interface. `#[doc(hidden)]` MUST NOT be treated as a privacy boundary.

A struct's fields SHOULD remain private when construction, mutation, or future representation changes require control. Public fields MAY be appropriate for simple records whose direct construction and representation are intentionally part of the contract. A getter and setter for every field are not an automatic improvement over a well-defined record.

Within a module, readers SHOULD encounter documentation and attributes, any necessary extern-crate declarations, imports and re-exports, module declarations, principal types and their implementations, supporting functions, and tests in a coherent order. Closely related definitions SHOULD remain close together. This is an organization preference, not a requirement to reorder items that have meaningful dependencies.

Field, variant, derive, and attribute order MUST NOT be changed mechanically without considering semantics. Field order can affect derived behavior and destruction order; procedural attributes can interact through expansion. A style tool or reviewer MUST NOT assume that all source ordering is cosmetic.[^rust-drop]

Inherent methods SHOULD be grouped by responsibility, with primary construction and common operations easy to find. Separate `impl` blocks MAY clarify distinct bounds or capabilities. A separate block for every method SHOULD NOT be used without a practical reason.

### D05. Bindings and control flow

Bindings MUST be immutable unless mutation is needed. The scope of a mutable binding or borrow SHOULD be no larger than the operation requiring it. Mutation SHOULD be direct and visible rather than hidden in a closure whose apparent purpose is transformation.

Shadowing MAY express a clear refinement, such as raw input becoming parsed input. It SHOULD NOT reuse a name for an unrelated concept or make the identity of a lock guard, resource, or authorization decision difficult to follow.

A value whose lifetime matters MUST have an intentional binding or explicit disposal. In particular, a wildcard discard is not a named guard. Use a named binding such as `_guard` when an RAII guard must remain alive; use `drop(value)` when early disposal is intended. The chosen scope MUST make the intended resource lifetime clear.[^rust-drop]

Use `match` for meaningful multi-way decisions, `if let` for one relevant pattern, and `let ... else` for a required shape followed by an early exit. Exhaustively match closed domain enums when adding a variant should force a review of the decision. A wildcard arm MAY be appropriate for an intentionally open-ended contract, but MUST NOT conceal an unhandled state that matters.

Prefer early returns for invalid inputs and exceptional branches. A function's final value SHOULD normally be a tail expression; explicit `return` is appropriate for early exits and MAY remain where it materially improves a complex function's clarity. Neither nesting depth nor line count is, by itself, a correctness or design criterion.

Iterator chains SHOULD express transformations. A `for` loop SHOULD express primarily effectful work, complicated branching, or stateful progress. Do not build a long chain merely to avoid a named intermediate value. Lazy adapters MUST NOT be used under the assumption that creating them executes their closures.[^std-iterator]

Use `collect::<Result<Vec<_>, _>>()` or an explicit fallible loop when failures must propagate. `.filter_map(Result::ok)`, `.flatten()` on fallible elements, and `.ok()` MUST NOT discard unexpected failures. Deliberately dropping failures requires a documented policy, not merely a convenient adapter.

### D06. Types, numbers, text, and collections

Types SHOULD encode distinctions whose confusion would cause a defect. Use newtypes for distinct identifiers or quantities, enums for mutually exclusive states, and validated types for values with stable invariants. A type alias improves vocabulary but MUST NOT be treated as a distinct type or a validation mechanism.

`Option<T>` MUST represent meaningful absence. `Result<T, E>` MUST represent success or failure. `Result<Option<T>, E>` is appropriate when absence and failure are different outcomes. Sentinel numbers, empty strings, and null-like conventions MUST NOT replace these types unless an external representation requires them.

A boolean parameter MAY represent an obvious binary property. Several adjacent booleans, a boolean with unclear polarity, or a mode expected to grow SHOULD be replaced by an enum or a named configuration type. A tuple return value is appropriate for a small, conventional grouping; otherwise use a named result type.

Integer types MUST match the domain. Use `usize` for in-memory indexing and sizes when appropriate, and explicitly sized integers for external formats with specified widths. Numeric literals SHOULD expose units through names or types, and long literals SHOULD use separators where they aid recognition.

A cast using `as` MUST be intentional about truncation, signedness, precision, and platform width. Use `From` for an appropriate infallible conversion and `TryFrom` when failure matters. A narrowing cast justified by a prior check MUST make that relationship clear. Where overflow is possible and consequential, choose checked, saturating, wrapping, or explicitly validated arithmetic according to the domain rather than relying on a build profile's incidental behavior.[^rust-operators][^std-from]

Floating-point code MUST define the treatment of non-finite values and comparison error when those affect correctness. Approximate equality SHOULD use a tolerance appropriate to the calculation and scale; an arbitrary universal epsilon is not a policy. Exact comparison MAY be correct for a specified representation, an exact operation, or a deliberately chosen sentinel.

Text, bytes, and paths MUST remain distinct. `String` and `str` are UTF-8 text; their lengths are byte lengths, and Rust `char` values are Unicode scalar values rather than necessarily user-perceived characters. Code MUST NOT assume that a byte offset is a valid string boundary.[^std-string]

Use `Path` and `PathBuf` for filesystem paths and `OsStr` and `OsString` where operating-system strings are required. Lossy text conversion MAY be used for an explicitly diagnostic display; it MUST NOT silently change identity, matching, or path resolution. An ASCII-only protocol MUST state and validate that assumption.

Collection choice SHOULD follow access patterns and required behavior. Observable output MUST NOT depend on unspecified hash-map or hash-set iteration order. Sort explicitly or use an ordered representation when reproducibility is part of the contract. Custom hashing and alternative collections require a reason beyond aesthetic preference.[^std-hashmap]

### D07. Ownership and borrowing

A parameter MUST ask for ownership only when the function consumes, retains, transforms, or otherwise benefits from owning the value. A borrowed parameter MUST NOT be immediately cloned merely because the signature was designed without considering retention.

| Intended operation | Usual parameter shape |
| --- | --- |
| Inspect an existing value | `&T` |
| Mutate an existing value without taking ownership | `&mut T` |
| Consume or retain a value | `T` |
| Read a sequence without requiring its allocation strategy | `&[T]` |
| Mutate existing sequence elements without resizing | `&mut [T]` |
| Read UTF-8 text | `&str` |
| Read a filesystem path | `&Path` |
| Support several cheap borrowed representations | A justified `AsRef<...>` bound |
| Accept an owned destination representation ergonomically | A justified `Into<...>` bound |

Prefer slices, `str`, and `Path` over `&Vec<T>`, `&String`, and `&PathBuf` when no container-specific capability is needed. `&mut Vec<T>` is appropriate when resizing is part of the operation. `&Box<T>` is appropriate only when the box itself, rather than merely its referent, is relevant.

Generics MUST NOT be added automatically to every borrowed parameter. A concrete `&str` can be a clearer contract than `impl AsRef<str>` when the extra accepted forms have no demonstrated value. Conversely, a generic `Read` or `Write` parameter taken by value can accept a mutable reference to an existing reader or writer through the standard implementations; it need not force consumption of the underlying resource.[^api-interoperability]

Cloning MAY be the correct design at an ownership boundary. Authors SHOULD avoid unnecessary clones, but MUST NOT replace a clear, bounded copy with intricate lifetimes or shared state solely to eliminate the word `clone`. Repeated or large copies on a relevant path require measurement or a documented bound.

For reference-counted handles, prefer `Arc::clone(&handle)` or `Rc::clone(&handle)` where making shared ownership explicit helps the reader. A clone of such a handle shares the referent; it is not a deep copy. The `Clone` trait does not universally promise either cheapness or deep copying.[^std-arc][^std-clone]

Lifetime elision SHOULD be used when it leaves the relationship clear. Explicit lifetimes MUST identify the relationship actually required, and unrelated borrows MUST NOT be tied together for convenience. A `'static` bound MUST reflect a real ownership or execution requirement, not a workaround for an unresolved lifetime error.

`Box::leak`, `mem::forget`, raw pointers, or lifetime-changing unsafe code MUST NOT be used merely to bypass ownership design. Intentional process-lifetime allocations or resource transfer MAY justify such mechanisms, but the resource and safety consequences require an explicit design decision.

### D08. Construction and validity

Construction MUST establish the invariants promised by the resulting type. All construction paths, including conversions, deserialization, builders, and internal helpers, MUST preserve the same contract unless their differing guarantees are explicit in their types and documentation.

Use `new` for a primary inherent constructor when it is useful, and descriptive alternatives for genuinely different construction modes. A constructor MAY return `Result`; the name `new` does not promise infallibility. A `try_` prefix MAY distinguish a fallible operation from an existing infallible counterpart, but SHOULD NOT be mechanically applied to every fallible function.

Implement `Default` only when there is a valid, unsurprising default value. `Default` MUST NOT create a temporarily invalid object that requires a later setter before ordinary use. When a no-argument `new` and `Default` represent the same operation, they SHOULD agree rather than encode two unexplained defaults.

Builders SHOULD be used for substantial optional configuration or a construction process clearer than a long argument list. A builder MUST validate cross-field constraints at a defined point, normally `build`. Setter names MUST identify their meaning, and ownership-taking setters MUST NOT unexpectedly duplicate large inputs.

Typestate MAY enforce an important construction or protocol sequence at compile time. It SHOULD NOT be used when a small runtime-validated type gives an equally clear contract with substantially less API complexity. Required fields, defaults, and failure behavior MUST remain discoverable without reading every implementation block.

### D09. Traits, generics, and conversions

A trait MUST represent a coherent behavioral contract. It SHOULD NOT exist solely to rename one concrete type or to satisfy a testing framework's preferred shape. Abstractions for storage, clocks, transports, or other genuine variation are appropriate when the variation matters to production design or reliable testing.

Generic parameters SHOULD represent supported independent choices. Associated types SHOULD represent types selected by an implementation's contract. Bounds MUST be no stronger than required; in particular, `Clone`, `Send`, `Sync`, `'static`, and `Sized` MUST NOT be added speculatively.

Choose concrete types for a fixed implementation, generics or argument-position `impl Trait` for suitable static polymorphism, and `dyn Trait` for deliberate type erasure or heterogeneous runtime selection. This is an API and cost decision, not a universal preference for one dispatch mechanism. A named generic parameter is appropriate when callers or other parameters must refer to the same type.

Return-position `impl Trait` MAY hide an implementation while retaining a statically known opaque type. Its exposed trait bounds, auto traits, and lifetime relationships are part of API review. Edition changes can affect implicit lifetime capture, so migration MUST review borrowed return values rather than assuming a formatting-only change.[^rust-185]

A trait intended for trait-object use MUST be designed and tested for dyn compatibility. In particular, a native `async fn` method is not directly dispatchable through `dyn Trait` under this document's Rust 1.85 baseline. Choose an appropriate future-returning or erased interface when both asynchronous behavior and dynamic dispatch are requirements.[^rust-traits]

Implement `From` and `TryFrom` rather than hand-writing the corresponding `Into` and `TryInto` conversions when the standard blanket implementations apply. `From` MUST be infallible and SHOULD describe an obvious, value-preserving conversion; use `TryFrom` for validation or other meaningful failure. Formatting and lossy reinterpretation MUST NOT be disguised as unsurprising `From` conversions.[^std-from]

Ad-hoc conversion names MUST follow the conventional ownership distinction: `as_` for an inexpensive borrowed view, `to_` for a conversion that does not consume a non-`Copy` receiver, and `into_` for a consuming conversion. These prefixes MUST NOT be treated as allocation guarantees: a `to_` operation can return a borrowed view, and an `into_` operation can reuse existing storage. Material validation or computation costs SHOULD be documented.[^api-naming]

Use `FromStr` for an appropriate conventional text parser, and a named parsing function when the operation needs extra context or an output borrowing from the input. Parsing MUST NOT be hidden in a supposedly infallible conversion.[^std-fromstr]

`AsRef` SHOULD expose an inexpensive borrowed view. `Borrow` MUST additionally satisfy the equivalence obligations relevant to equality, ordering, and hashing; it is not merely a differently named `AsRef`.[^std-borrow]

Implement standard traits only when their semantics fit. `Eq`, `Ord`, and `Hash` implementations MUST agree with their documented relationships. A custom ordering MUST NOT contradict the corresponding equality. `Copy` MUST be a deliberate value-semantics commitment, not a way to suppress ownership errors. Public types SHOULD implement a useful, safe `Debug`; sensitive fields require a redacted implementation rather than indiscriminate deriving.[^api-interoperability][^std-hash]

`Display` SHOULD be reserved for an intentional user-facing representation. Code MUST NOT parse `Debug` or `Display` output as a stable format unless stability is explicitly part of that contract. Operator traits MUST preserve recognizable operator meaning. `Deref` and `DerefMut` SHOULD model pointer-like access, not simulate inheritance or expose an unrelated convenience API.[^api-predictability]

Use `#[must_use]` on a returned value, type, or operation when discarding it is likely to indicate misuse and the existing type does not already communicate that obligation. Add a message when the consequence is not obvious. An attribute is a diagnostic aid, not a runtime enforcement or safety mechanism.[^rust-diagnostics]

Public extension traits SHOULD have a discoverable scope and a meaningful name. A trait MAY be sealed when downstream implementations would prevent the crate from maintaining its intended contract; sealing MUST be intentional and documented, not an accidental result of inaccessible bounds.

### D10. Errors and panics

Expected operational failure MUST be expressed through `Result` or an equivalent documented error-bearing interface. A reusable API SHOULD expose a meaningful error type or an established standard error appropriate to its contract. Public errors MUST NOT be bare strings or `()` when callers need interpretation, chaining, or a useful diagnostic. A type-erased error MAY be appropriate at an application orchestration boundary; a general-purpose library SHOULD NOT erase distinctions that its callers need.

Error types SHOULD implement `core::error::Error` or `std::error::Error` as appropriate, with meaningful `Display` and `Debug` implementations. When an error wraps a causal error, it MUST preserve that cause through the supported error-chain mechanism unless the cause is intentionally excluded for security or interface reasons.[^std-error]

Error messages SHOULD describe the failed operation or violated constraint, include useful non-sensitive context, and omit redundant prefixes such as “error:”. Composable error fragments SHOULD start in lowercase and omit terminal punctuation, except where grammar or a proper name calls for otherwise. A machine decision MUST NOT depend on the wording of a human-readable message.

Use `?` for straightforward propagation. Use `map_err` or an explicit `match` when adding meaningful context or changing abstraction boundaries. Do not wrap an error repeatedly with identical text. A layer SHOULD either handle a failure or propagate it; merely logging and returning the same failure usually duplicates the diagnostic.

An ignored error MUST have a deliberate policy. `.ok()`, `let _ = operation()`, and fallback helpers MUST NOT be used solely to quiet an unused-result warning. A fallback is appropriate only when the fallback is valid for that specific failure class and its observability requirements are met.

A panic is appropriate for an internal invariant violation or a deliberately documented programmer-error contract. It MUST NOT be the default response to malformed external input, ordinary I/O failure, an absent optional resource, or an expected service failure. Libraries MUST NOT terminate the process to avoid returning an error.

`expect` MAY be used where an invariant establishes that failure is a programming defect and a panic is the chosen policy. Its message SHOULD state why success is expected. `unwrap` MAY be used in tests and small, self-evident invariant cases where an additional message would add no information. Neither method is permitted merely because a failure is unlikely.

Production paths MUST NOT contain reachable `todo!` or `unimplemented!`. `unreachable!` MUST correspond to an actual invariant; an enum variant not yet implemented is not unreachable. Prefer exhaustive type-driven handling over a panic that hides an incomplete state model.

`debug_assert!` MUST NOT be the only enforcement of a condition needed for memory safety, authorization, or required release behavior. `catch_unwind` MUST NOT be used as ordinary error handling or as a promise to recover from every panic strategy. Cleanup and recovery designs MUST account for their actual unwinding or abort behavior.[^std-panic]

### D11. Documentation and comments

Every reachable public item MUST have useful documentation, whether directly written or intentionally inherited from a documented trait contract. Crate documentation MUST explain purpose, primary usage, relevant capabilities, and significant constraints. Public documentation MUST be understandable without access to the implementation.

A function's documentation SHOULD begin with a concise description of what it does. It MUST document meaningful input requirements, return semantics, externally observable effects, and limitations. Add `# Errors`, `# Panics`, and `# Safety` sections when those topics apply; do not add empty boilerplate sections. Every public unsafe function and unsafe trait MUST have a `# Safety` section that states the caller's or implementer's obligations.[^api-documentation]

Resource-intensive, blocking, or asynchronous interfaces MUST document relevant cost, blocking, cancellation, and partial-progress behavior. A method returning a borrowed value MUST explain unusual validity or invalidation constraints that are not clear from its signature. Public feature and platform restrictions MUST be discoverable in rendered documentation.

Use `///` for item documentation and `//!` for crate or module documentation. Names of Rust items SHOULD be code-formatted and linked using rustdoc intra-doc links where useful. Broken intra-doc links MUST be corrected, not silenced with broad lint suppression.[^rustdoc-links]

Examples SHOULD demonstrate ordinary correct use, including error propagation and any essential cleanup. Use runnable doctests whenever practical. `no_run` is appropriate when an example must compile but cannot execute safely or reliably in the test environment; `ignore` requires a specific documented reason. Hidden setup lines MAY remove incidental scaffolding but MUST NOT conceal steps essential to safe or correct use.[^rustdoc-tests]

Compile-fail examples SHOULD demonstrate a particular rejected misuse. Their validation MUST check that failure occurs for the intended reason, rather than because an import or unrelated type is missing. Examples that depend on features or external tools MUST identify that dependency.

Implementation comments SHOULD explain why a decision exists, which invariant a transformation preserves, or why an apparently simpler alternative is wrong. They SHOULD NOT paraphrase obvious syntax. Comments MUST be updated when their associated behavior changes.

A `SAFETY:` comment MUST be a local argument, not a label attached to an unexplained unsafe operation. A `TODO` or `FIXME` SHOULD identify a concrete remaining action and a tracking reference or accountable owner. Commented-out alternative implementations SHOULD be removed rather than maintained as informal history.

Documentation prose, comments, diagnostics intended for developers, and identifiers SHOULD use consistent English terminology. Test fixtures and user-visible localized content MAY use other languages when the behavior requires it.

### D12. Resources and I/O

Resources MUST have an identifiable owner and a defined release policy. RAII SHOULD handle ordinary cleanup. When completion can fail, provide an explicit operation such as `finish`, `flush`, `close`, or `commit` that reports failure rather than relying only on `Drop`.

Destructors MUST NOT intentionally panic as an ordinary failure-reporting mechanism. A destructor that can perform substantial blocking work MUST document that behavior and SHOULD have an explicit alternative. Safety MUST NOT depend on every destructor running: safe Rust permits values to be forgotten, and process termination can bypass ordinary destruction.[^std-forget]

An I/O API MUST define whether success means bytes were accepted, buffers were flushed, data was synchronized, or a higher-level transaction completed. These are distinct guarantees. A single `write` may accept only part of its input; use `write_all` when the contract requires the complete buffer to be written, while still documenting the possibility of partial effects before an error.[^std-write]

Untrusted lengths, counts, and decompression or parsing work MUST be bounded before they control resource use. Code MUST NOT read an unbounded stream into memory by default unless an external bound is established and documented. Retrying I/O MUST account for which effects may already have occurred.

Reusable libraries SHOULD accept explicit handles, paths, configuration, and collaborators rather than implicitly reading process-wide state. They MUST NOT write unrelated output to stdout or stderr, install global logging handlers, or change process settings as a hidden side effect of ordinary construction.

Configuration SHOULD be loaded at an application boundary and passed inward. Tests SHOULD use explicit configuration or child-process configuration rather than mutating process-global environment state. In Edition 2024, environment mutation APIs such as `std::env::set_var` and `remove_var` are unsafe; wrapping them in an unsafe block is not a substitute for satisfying their platform safety contracts.[^edition-env]

### D13. Concurrency and shared state

Each shared mutable resource MUST have a stated synchronization strategy. Prefer transferring ownership or confining state to one owner when that expresses the design clearly. Use shared immutable state when mutation is unnecessary. Do not introduce shared mutability merely because several functions need to read the same data.

Use `Rc` for appropriate single-threaded shared ownership and `Arc` when shared ownership must cross threads. `Arc<T>` does not make an arbitrary `T` safe for concurrent access; its thread-safety properties depend on the referent. Interior mutability and synchronization MUST match the access pattern.[^std-arc]

Lock scope SHOULD be short and visible. Locks MUST NOT be held while invoking unknown callbacks or unrelated I/O unless the contract explicitly requires it and the deadlock and latency risks have been reviewed. When multiple locks can be held together, their acquisition order MUST be defined and followed.

A blocking mutex guard MUST NOT be held across `.await` in code conforming to this standard. Move synchronous state access into a clearly delimited scope or helper. An asynchronous mutex MAY be held across an await only when the operation genuinely requires it and cancellation and deadlock behavior are understood. A synchronous mutex can still be appropriate for short, non-awaiting, low-contention access in asynchronous code; “async code” does not automatically require an async mutex.[^tokio-state]

Poisoning MUST have an explicit response: propagate, fail according to a documented invariant policy, or recover after checking or restoring state. Ignoring poisoning without examining the protected invariant MUST NOT be treated as recovery.

Every use of atomics MUST have a clear state model. Ordering weaker than `SeqCst` requires an explanation of the relevant synchronization or independence argument. `Relaxed` is appropriate only when atomicity without inter-thread ordering is sufficient; `SeqCst` alone does not prove an algorithm correct.[^std-ordering]

Threads, channels, queues, and worker pools MUST have defined ownership and termination behavior. Unbounded queues or unconstrained thread creation require a documented external bound or an approved resource policy. Closure of a channel MUST be handled as part of the protocol, not treated universally as an impossible event.

### D14. Asynchronous code

Use async interfaces for appropriate asynchronous operations, not as a cosmetic wrapper around blocking work. Polling a future is expected to return promptly; substantial blocking or CPU work MUST be assigned to a suitable execution mechanism. A future is not automatically a separately executing task.[^std-future]

Every spawned task MUST have an owner responsible for observing completion, handling failure, and initiating or coordinating shutdown. A detached task MAY be used only when its lifetime, resource bounds, failure reporting, and shutdown behavior are deliberately specified. Dropping a handle MUST NOT be assumed to stop its work: for example, dropping a Tokio `JoinHandle` detaches the task.[^tokio-join]

Every `.await` MUST be treated as a possible suspension point. State that remains visible during suspension MUST satisfy its required invariants. Code SHOULD avoid retaining large temporary buffers, mutable borrows, or scarce resources across an await when they are no longer needed.

Cancellation behavior MUST be part of the contract when interruption can leave externally visible progress. Dropping a future stops further polling of that value; it does not necessarily stop external work, roll back an operation, or cancel a separately spawned task. A selection construct's ownership of each future matters, so authors MUST verify the semantics of the actual combinator being used.[^std-future][^tokio-select]

Before applying a timeout, racing operations, or retrying a canceled operation, determine what can already have happened. An operation that is memory-safe to drop is not necessarily safe to retry or abandon at the protocol level. Required asynchronous cleanup MUST have an explicit shutdown path; a destructor alone MUST NOT be assumed to perform it.

Concurrency limits, queue bounds, retry limits, and deadline ownership MUST be explicit in services that can accumulate work. A timeout MUST NOT be presented as a hard execution bound when the underlying operation is blocking or otherwise cannot be interrupted as assumed.

A public async trait MUST specify the mobility requirements of its returned futures when callers rely on cross-thread execution. A `Send` implementation type or a `Sync` receiver alone MUST NOT be assumed to imply that every returned future is `Send`. Runtime-specific types SHOULD remain at integration boundaries unless the crate intentionally exposes that runtime as part of its purpose.

Tests MUST cover relevant cancellation, shutdown, and partial-progress scenarios, not only a future that immediately completes successfully. Test synchronization SHOULD use explicit events or controlled time rather than timing guesses.

### D15. Unsafe code

Unsafe code requires a specific need: interoperability, a low-level abstraction not expressible with the required properties in safe Rust, or a demonstrated performance requirement for which an appropriate safe alternative is insufficient. Convenience, anticipated speed, or discomfort with the borrow checker is not sufficient.

Unsafe operations MUST be isolated behind the smallest coherent abstraction that can maintain their invariants. This does not mean arbitrarily tiny blocks with no surrounding argument; the boundary must be small enough to audit and large enough to own the invariant.

Every unsafe block MUST have an adjacent `SAFETY:` explanation covering the operation's applicable preconditions. Every unsafe function or trait MUST document the obligations it places on callers or implementers. Every unsafe implementation, including manual `Send` or `Sync`, MUST explain why the type's full safe interface preserves the claimed property.

The repository MUST deny `unsafe_op_in_unsafe_fn`. Unsafe operations inside an unsafe function MUST appear in explicit unsafe blocks, separating the function's caller contract from the implementation's local proof obligations. Edition 2024 enables a warning for this distinction; this standard deliberately strengthens it to a denied lint.[^edition-unsafe-op]

A safety review MUST identify the source of each relevant guarantee: allocation validity, alignment, initialization, bounds, aliasing, lifetime, thread access, representation, and behavior during errors, panics, or cancellation. “The caller knows” or “the tests pass” is not a sufficient argument. The actual obligations depend on the operations used; a generic checklist does not replace their API contracts.

Safe callers MUST NOT be able to invalidate a hidden safety assumption through an otherwise permitted method, a trait implementation, reentrancy, or a forgotten guard. Safety invariants MUST be enforced by visibility, types, runtime checks, or an explicit unsafe caller contract. A safe method's documentation-only precondition can justify a controlled failure, but not undefined behavior.

Representation-dependent code MUST rely only on documented layout guarantees. Equal sizes do not establish that a `transmute` is valid. Raw memory handling MUST account for invalid bit patterns, padding, partial initialization, and ownership. `MaybeUninit`, pointer casts, and `repr(C)` are tools with specific contracts, not general permission to reinterpret bytes.[^rust-layout][^std-maybeuninit]

Use established safe abstractions before introducing custom pointer or pinning machinery. A pinned pointer does not by itself make arbitrary self-referential code sound; projection, movement, and destruction obligations still require a complete argument.[^std-pin]

Unsafe code MUST receive review by a maintainer competent in the relevant invariants. Appropriate dynamic analysis, including Miri where supported, SHOULD supplement ordinary tests. A passing tool run MUST NOT be represented as a proof of soundness, and undocumented assumptions about still-evolving aliasing semantics MUST NOT be presented as settled language guarantees.[^miri][^rust-ub]

Crates intended to contain no unsafe code SHOULD use `#![forbid(unsafe_code)]`. This restricts that crate's source; it is not a claim that its dependencies contain no unsafe code. Workspaces that contain deliberate low-level components MAY instead deny unsafe code by default and permit it in explicitly reviewed scopes.

### D16. Foreign-function interfaces

FFI MUST be isolated in a clearly identified layer. Raw bindings SHOULD be separated from the safe Rust interface. The wrapper MUST establish the foreign function's preconditions and translate its ownership, error, and lifetime conventions into a usable Rust contract.

Declarations MUST use the correct ABI and types for the supported foreign interface. Edition 2024 requires `unsafe extern` blocks. Marking the block unsafe records the declaration author's responsibility; it does not verify a declaration against the foreign library.[^edition-extern]

An interface MUST specify nullability, buffer lengths, alignment, ownership transfer, allocator pairing, callback lifetime, reentrancy, and thread restrictions whenever relevant. Rust enums, references, strings, and container types MUST NOT be assumed to match a foreign representation merely because a test works on one platform. Invalid external discriminants MUST be validated before they become Rust enum values.

Public C-facing records SHOULD use an appropriate documented representation and C-compatible fields. `repr(C)` MUST NOT be treated as making every field type FFI-safe. Opaque handles SHOULD be used where exposing layout would unnecessarily constrain the implementation.

Unwinding across a language boundary MUST have an explicit ABI and runtime policy. A panic MUST NOT accidentally escape through a boundary that does not permit it. `catch_unwind` can support a carefully designed Rust-unwind boundary, but MUST NOT be advertised as catching aborts or arbitrary foreign exceptions.[^rust-ffi]

Exported symbols and special linking attributes MUST have a collision and ownership policy. In Edition 2024, attributes such as `no_mangle`, `export_name`, and `link_section` use the `unsafe(...)` form and MUST receive a safety explanation where applied.[^edition-unsafe-attrs]

### D17. Macros, attributes, and conditional compilation

Use an ordinary function, type, or trait when it expresses the operation adequately. Macros MAY be used for meaningful syntax abstraction, repetitive declarations, or compile-time generation that would otherwise reduce clarity or maintainability.

A macro MUST make evaluation order, expression evaluation count, control-flow effects, and unsafe obligations predictable. Expression arguments MUST NOT be evaluated multiple times unless that behavior is essential, explicit in the interface, and documented. An apparently safe macro MUST NOT expose unchecked unsafe behavior to safe callers.

Exported declarative macros MUST use hygienic paths and appropriate `$crate` references for defining-crate items rather than relying on the caller's incidental imports. Their helper visibility and re-export behavior MUST be tested from a separate consumer crate. `$crate` does not waive visibility rules.[^rust-macros]

Macros SHOULD accept conventional trailing commas and provide useful errors for invalid input. Procedural macros SHOULD report diagnostics at relevant input spans. Their generated API, trait bounds, unsafe code, and runtime costs MUST be reviewed as part of the library's actual interface.

Attributes MUST be placed on the narrowest item expressing their purpose. Derives SHOULD be used when their semantics match the contract; a derive is not a substitute for reviewing that contract. Attribute reordering MUST account for procedural expansion semantics.

Use `#[cfg(...)]` to include or exclude items or code. `cfg!(...)` produces a boolean and MUST NOT be used under the assumption that the unselected branch escapes name resolution or type checking. Platform-dependent logic SHOULD be concentrated behind a coherent module boundary rather than scattered through business logic.[^rust-cfg]

Every custom configuration name MUST be registered with the project's supported check-cfg mechanism. Unexpected configuration warnings MUST NOT be broadly suppressed. Feature and target conditions MUST be exercised through the configuration matrix in [D20](#d20-tests-and-examples).

### D18. Toolchains and Cargo configuration

Every first-party package MUST declare its edition and MSRV, directly or through explicit workspace inheritance. A virtual workspace MUST declare its resolver. New Edition 2024 workspaces adopting this standard use resolver `"3"` unless a documented compatibility constraint requires another choice.[^cargo-workspaces][^edition-resolver]

The exact toolchain used for formatting and the primary lint gate MUST be pinned in repository-controlled configuration. A floating `stable` channel is suitable for a separately managed compatibility check, not for claiming that formatting output is reproducible over time. Toolchain updates MUST be reviewed and MUST include required configuration or source updates.

The MSRV MUST be tested independently from the development toolchain. A successful build with a newer compiler is not evidence of MSRV compatibility. The support policy MUST state whether it covers tests and development tools as well as the consumable library, and how dependency changes that affect the MSRV are handled.[^cargo-msrv]

Shared edition, lint, and dependency policy SHOULD be placed in workspace configuration. Each member MUST explicitly inherit applicable settings; the existence of `[workspace.lints]` alone does not apply those lints to every member.[^cargo-workspaces]

Manifests SHOULD keep package identity, target definitions, features, dependencies, and lint policy in a consistent order. Dependency entries within a section SHOULD be sorted by their local names. Manifests MUST NOT retain unused keys, obsolete commented-out dependency alternatives, or unexplained patches.

Private packages MUST set `publish = false`. Published packages MUST have accurate metadata, a suitable README, a documented license policy, and correct package contents. A release's archive MUST be checked rather than assuming every required runtime or generated file is included.[^cargo-manifest]

Build scripts MUST declare relevant file and environment inputs through supported Cargo rerun directives. Generated output belongs in `OUT_DIR` unless a separately documented generation workflow deliberately maintains checked-in files. Build scripts MUST NOT silently modify source files, depend on a developer's home directory, or require unrecorded network access.[^cargo-build]

### D19. Dependencies, lockfiles, and features

Each new dependency MUST have a justified purpose and an accountable maintenance decision. Review SHOULD consider API fit, supported toolchains and targets, enabled features, transitive cost, provenance, and the project's licensing and security requirements. A dependency's popularity is not an adequate substitute for that review.

Use normal compatible version requirements unless exact pinning or a narrower constraint is necessary and documented. Wildcard version requirements MUST NOT be used. Git dependencies require an immutable revision and an approved reason; a moving branch MUST NOT be the sole definition of a reproducible release input.

First-party workspaces MUST commit `Cargo.lock`, including library-only workspaces, unless a repository-specific exception defines an alternative reproducibility strategy. This is an organizational rule consistent with Cargo's lockfile guidance. A library's repository lockfile controls its own development resolution; it does not constrain every downstream consumer's dependency resolution.[^cargo-lock]

Primary CI and release validation MUST use the intended checked-in dependency resolution. `--locked` SHOULD be used on relevant Cargo commands to reject an unintended lockfile update. A lockfile is necessary for this policy but MUST NOT be described as sufficient for a fully reproducible build independent of compiler, native tools, environment, and build-script inputs.

Libraries SHOULD also test dependency updates or representative downstream resolution separately, so a known-good repository lockfile does not conceal a broken published requirement. Such checks MUST NOT silently rewrite the lockfile used by the primary conformance gate.

Features MUST represent documented capabilities. Names SHOULD describe the capability directly, such as `serde`, `compression`, or `std`, rather than filler names such as `use-serde`. Prefer additive features: enabling one SHOULD add a capability without disabling another or weakening correctness. Cargo's feature unification makes mutually exclusive dependency features difficult to compose.[^cargo-features]

Default features MUST be deliberate and documented. A small default set is useful only if it matches expected use; disabling all defaults is not automatically a better library design. Optional dependencies that are implementation details SHOULD be exposed through intentional feature definitions using `dep:` where appropriate.

Mutually exclusive features require an architectural justification and an explicit failure for unsupported combinations. Consider separate crates or runtime selection before imposing that restriction. Features MUST NOT disable safety checks, authorization, or required validation as an undocumented performance mode.

Dependency defaults can be re-enabled elsewhere in a graph; specifying `default-features = false` on one edge MUST NOT be treated as proof that a dependency's defaults are globally absent. Tests MUST cover the actual feature combinations the package promises.[^cargo-features]

### D20. Tests and examples

Tests MUST validate the behavior and invariants promised by the abstraction. They SHOULD focus on observable outcomes rather than incidental call ordering, private field layout, or a particular implementation strategy.

Unit tests SHOULD be colocated in a `#[cfg(test)] mod tests` when they benefit from private access. Integration tests SHOULD exercise the external interface. Examples and doctests SHOULD represent supported usage rather than a separate, unmaintained presentation layer.

Test names MUST describe the relevant condition and outcome in `snake_case`, such as `rejects_a_zero_batch_size`. Table-driven cases SHOULD include identifying input or case names in assertion failures. A test that fails without explaining which case failed is unnecessarily difficult to maintain.

Tests MUST be independent of execution order and SHOULD be safe to run concurrently. They MUST NOT rely on a developer's filesystem, personal credentials, machine locale, or access to an external service unless explicitly classified as environment-dependent integration tests. Tests of such integrations require their own documented setup and isolation.

Use controlled clocks, deterministic fixtures, explicit synchronization, and reproducible random seeds where appropriate. Sleeping for an assumed scheduling interval MUST NOT be used as the primary proof that a concurrent action occurred. Snapshot tests SHOULD normalize irrelevant variation without concealing meaningful behavior.

Failure tests MUST check the intended failure class, not merely that “something failed.” Regression tests SHOULD isolate the reason for the regression. Expected-panic tests MUST be used only for a deliberately panicking contract and SHOULD narrow the expected cause.

Each supported configuration MUST have a validation strategy. At minimum, the matrix MUST address the default feature set, the minimal supported feature set, important optional capabilities, relevant interactions, the MSRV, and supported platform-specific code. Not every project needs an exhaustive power set of feature combinations; the selected coverage and exclusions MUST be explicit and risk-based.

`--all-features` MUST NOT be treated as a substitute for testing without defaults or individual feature boundaries. `--all-targets` selects Cargo target kinds, not all CPU or operating-system targets. An all-targets test run does not replace a separate doctest run; use the appropriate test commands for both.[^cargo-test]

Property tests SHOULD exercise algebraic laws and invariants where examples alone are weak. Parsers, unsafe abstractions, and state machines SHOULD receive fuzzing or specialized analysis proportionate to their risk. Tests that assert public auto-trait properties or compile-time usage constraints are appropriate when those properties are part of the contract.

Ignored tests require an explanation and a documented execution path. A permanently ignored failing test MUST NOT be used to suggest that a requirement is covered. Benchmarks MUST be separated from correctness assertions that depend on machine-specific timing.

### D21. Performance

Performance changes MUST preserve documented behavior and MUST have an identified target: latency, throughput, memory, allocation rate, code size, startup cost, or build cost. Changes that add substantial complexity SHOULD include measurements against a representative baseline.

Measurements MUST identify relevant workload, build profile, target, and material environmental conditions. An optimized microbenchmark MUST NOT be presented as proof of an end-to-end improvement it does not measure. Results SHOULD include enough information to reproduce the comparison and distinguish noise from a meaningful change.

Improve algorithms, data movement, allocation patterns, and I/O behavior before introducing unsafe indexing, manual vectorization, or custom synchronization. Reserve capacity when a justified bound or estimate exists; do not trust an arbitrary external length merely to avoid reallocation.

`#[inline(always)]`, custom allocators, packed representations, unchecked access, and similar mechanisms MUST NOT be applied as general style defaults. The presence of a generic function or iterator chain MUST NOT be used as an unsupported claim that the implementation has no runtime cost.

A performance-sensitive implementation SHOULD retain a clear explanation of the invariant or workload that justifies its complexity. A later maintainer should be able to tell when the optimization is still useful and which tests protect its behavior.

### D22. Portability and `no_std`

A crate MUST document its supported targets and meaningful platform assumptions. It MUST NOT claim portability based solely on avoiding obviously platform-specific imports. Pointer width, endianness, atomic capabilities, filesystem behavior, path encoding, clocks, and native dependencies can affect portability.

External formats MUST specify integer widths and byte order. Native Rust layout and `usize` MUST NOT be used as an undocumented portable wire format. Platform behavior SHOULD be isolated behind interfaces that expose the capability rather than spreading target-specific assumptions through domain code.

`no_std` support is optional. A crate that claims it MUST validate that claim in a suitable build configuration and target environment. Merely passing `--no-default-features` on a host that still links `std` elsewhere is not sufficient evidence of the complete claim.

An optionally standard-library-dependent crate SHOULD use a positive `std` feature. Allocation support MUST be documented separately from standard-library support; `no_std` does not inherently mean “no allocator.” Dependencies and their enabled features MUST support the advertised environment.[^cargo-features]

Embedded, WebAssembly, and other constrained targets MUST have an explicit policy for allocation, panic behavior, blocking, global state, and resource limits where relevant. Libraries SHOULD avoid choosing an application's global allocator, panic handler, executor, or process policy unless supplying that integration is their stated purpose.

### D23. API evolution and compatibility

A public change MUST be reviewed for source, behavioral, and configuration compatibility. Visibility, trait bounds, implemented traits, auto traits, lifetime relationships, feature defaults, dependency types in signatures, and error variants can all affect downstream code. Compatibility review MUST NOT consider only function names and argument counts.[^cargo-semver]

A crate SHOULD avoid exposing third-party types solely because they are convenient internally. When exposure is intentional, the dependency becomes part of the API's compatibility considerations. A wrapper SHOULD be introduced only when it provides a useful stable boundary, not merely an extra layer with identical commitments.

`#[non_exhaustive]` MAY be used when future growth of a public enum or record is intentional. It MUST be chosen as part of the initial API design where practical; adding it later can itself break consumers. It SHOULD NOT be applied indiscriminately to avoid deciding what the abstraction promises.[^cargo-semver]

Deprecations MUST describe the replacement or explain why no replacement exists. Releases MUST document meaningful behavior changes, MSRV changes, feature changes, and migration requirements. Removing an inconvenient public behavior without notice is not a style cleanup.

Serialization and protocol compatibility MUST be reviewed independently of Rust source compatibility. Deriving a serializer does not by itself establish a stable external schema. Field renaming, enum representation, defaults, and unknown-field handling MUST be intentional when stored or transmitted data outlives the running process.

### D24. Generated code and third-party boundaries

Generated files MUST be clearly identified, and their source inputs, generator version, and regeneration command MUST be discoverable. Reproducible generation SHOULD produce a clean working tree when rerun with the approved inputs and toolchain.

Generated code SHOULD be formatted when feasible. When a generator cannot satisfy first-party presentation or lint policy, the exception MUST be confined to generated output. Hand-written wrappers and generator source remain subject to the ordinary standard.

Authors MUST NOT manually fix generated output as the sole permanent change. Update the generator or inputs and regenerate. A temporary emergency patch requires a tracking record and a plan to restore reproducible generation.

Vendored code SHOULD retain upstream conventions to keep updates and patches reviewable. First-party modifications MUST be identifiable. A third-party exception MUST NOT become a reason to copy weak conventions into adjacent first-party code.

### D25. Diagnostics and sensitive data

Diagnostics MUST provide useful operational context without exposing secrets or unnecessarily disclosing personal data. Derived `Debug`, error chains, tracing fields, panic messages, and test snapshots MUST all be considered possible disclosure paths. Redaction MUST happen before a value reaches an ordinary diagnostic sink.

Libraries SHOULD expose errors and optional structured events rather than configuring global output. Applications SHOULD select logging and tracing policy at their boundary. A failure SHOULD be reported once at the layer that has enough context and responsibility to act on it.

Log levels MUST have documented project meanings. High-volume loops MUST NOT emit unbounded diagnostics as an accidental consequence of routine failure. Expensive diagnostic preparation SHOULD be avoided when the event will not be recorded.

Committed production paths MUST NOT contain incidental `dbg!` calls or temporary print debugging. Diagnostic output MUST NOT be relied on as a machine-readable protocol unless that protocol is explicitly designed and versioned.

Security-sensitive code MUST use explicit validation and resource limits and MUST preserve the distinction between parsing, validation, and authorization. Memory-safe code is not automatically free from denial-of-service, logic, or confidentiality defects. This style standard does not replace a threat model or a security review.

## Best Practices

This part is **advisory**. Its examples illustrate ways to satisfy the normative rules; they do not require every crate to adopt the illustrated architecture, helper, or representation.

### B01. Validate once at a domain boundary

A validated type can move a repeated runtime question into a construction contract. This is useful when several operations require the same invariant and callers otherwise have to remember it independently. It is less useful when the type introduces vocabulary without preventing a plausible mistake.

The following type admits only batch sizes from 1 through 4,096. Its field is private, its conversion reports a typed error, and its accessor cannot invalidate the range. The explicit upper bound is an example policy, not a Rust-wide limit.

```rust
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;

const MAX_BATCH_SIZE: usize = 4_096;

/// A batch size between 1 and 4,096, inclusive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BatchSize(NonZeroUsize);

impl BatchSize {
    /// Returns the validated number of items in a batch.
    pub fn get(self) -> usize {
        self.0.get()
    }
}

/// A value outside the supported batch-size range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidBatchSize {
    value: usize,
}

impl fmt::Display for InvalidBatchSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "batch size {} is outside 1..={MAX_BATCH_SIZE}",
            self.value
        )
    }
}

impl Error for InvalidBatchSize {}

impl TryFrom<usize> for BatchSize {
    type Error = InvalidBatchSize;

    /// Validates that the size is within the supported range.
    ///
    /// # Errors
    ///
    /// Returns an error when `value` is zero or greater than 4,096.
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        NonZeroUsize::new(value)
            .filter(|size| size.get() <= MAX_BATCH_SIZE)
            .map(Self)
            .ok_or(InvalidBatchSize { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_sizes_within_the_range() {
        for value in [1, 64, MAX_BATCH_SIZE] {
            assert_eq!(
                BatchSize::try_from(value).map(BatchSize::get),
                Ok(value),
                "input: {value}"
            );
        }
    }

    #[test]
    fn rejects_sizes_outside_the_range() {
        for value in [0, MAX_BATCH_SIZE + 1, usize::MAX] {
            assert_eq!(
                BatchSize::try_from(value),
                Err(InvalidBatchSize { value }),
                "input: {value}"
            );
        }
    }
}
```

This example deliberately has no arbitrary `Default` and no mutable access to the inner integer. Adding deserialization later would require routing it through equivalent validation rather than assuming that deriving the implementation preserves the invariant. See [D06](#d06-types-numbers-text-and-collections), [D08](#d08-construction-and-validity), and [D23](#d23-api-evolution-and-compatibility).

### B02. Borrow when the caller retains ownership

The function below searches existing entries and returns a borrowed view. The result is tied to the entries, not to the temporary lookup key. The explicit lifetime documents that distinction because there are two borrowed inputs.

```rust
/// Returns the first value whose name equals `key`.
///
/// The returned value borrows from `entries`. The search uses exact,
/// case-sensitive string equality and does not allocate.
pub fn find_value<'entries>(
    entries: &'entries [(String, String)],
    key: &str,
) -> Option<&'entries str> {
    for (name, value) in entries {
        if name == key {
            return Some(value.as_str());
        }
    }
    None
}
```

This interface needs neither an owned key nor a cloned result. That does not make borrowing universally superior: an API that stores the key or sends the result to independently owned work may correctly choose ownership. See [D07](#d07-ownership-and-borrowing).

### B03. Preserve failure during a transformation

A transformation that parses several values often needs to fail rather than quietly produce a shorter collection. A fallible `collect` expresses that policy without a manual accumulator. The example returns the first parse failure; a user-facing importer might instead collect indexed diagnostics for all invalid records.

```rust
use std::num::ParseIntError;

/// Parses each input as an unsigned 32-bit integer.
///
/// # Errors
///
/// Returns the first parsing error. No partial output is returned.
pub fn parse_counts(inputs: &[&str]) -> Result<Vec<u32>, ParseIntError> {
    inputs.iter().map(|input| input.parse()).collect()
}

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;

    #[test]
    fn preserves_all_valid_values() {
        assert_eq!(parse_counts(&["3", "5"]), Ok(vec![3, 5]));
    }

    #[test]
    fn reports_an_invalid_value_instead_of_skipping_it() {
        let error =
            parse_counts(&["3", "invalid", "5"]).expect_err("the second input is not an integer");
        assert_eq!(error.kind(), &IntErrorKind::InvalidDigit);
    }
}
```

Replacing the mapping with `filter_map(|input| input.parse().ok())` would implement a different policy: accept the valid subset and discard the rest. Such a policy can be legitimate, but it deserves an explicit API contract and appropriate reporting. See [D05](#d05-bindings-and-control-flow) and [D10](#d10-errors-and-panics).

### B04. Separate writing from completion guarantees

A small generic I/O function can avoid unnecessary ownership assumptions while documenting exactly what success means. Here the protocol is a four-byte big-endian payload length followed by the payload, with a one-mebibyte application limit.

```rust
use std::io::{self, Write};

const MAX_FRAME_LEN: usize = 1_048_576;

/// Writes a four-byte big-endian length followed by `payload`.
///
/// Payloads may contain at most 1,048,576 bytes. This function does not
/// explicitly call `flush` or synchronize persistent storage. An owned
/// writer may run its destructor on return; pass a mutable reference
/// to retain ownership.
///
/// # Errors
///
/// Returns `InvalidInput` before writing if the payload is too large.
/// Otherwise, returns the writer's error. An I/O error may leave a
/// partial frame in the destination; retrying the whole frame is not
/// automatically safe for the surrounding protocol.
pub fn write_frame<W: Write>(mut writer: W, payload: &[u8]) -> io::Result<()> {
    if payload.len() > MAX_FRAME_LEN {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "payload exceeds the frame-size limit",
        ));
    }

    let len = u32::try_from(payload.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "payload length does not fit the frame header",
        )
    })?;

    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_length_before_the_payload() -> io::Result<()> {
        let mut output = Vec::new();
        write_frame(&mut output, b"abc")?;
        assert_eq!(output.as_slice(), b"\0\0\0\x03abc");
        Ok(())
    }
}
```

A production implementation would add tests with short-writing and error-injecting writers, invalid sizes, and an empty payload. In the demonstrated call, the caller retains its writer and chooses the larger transaction's completion policy. See [D12](#d12-resources-and-io).

### B05. Keep a lock outside an asynchronous operation

For a cache-backed asynchronous operation, a useful decomposition is to obtain an owned request snapshot under a short lock, release the guard, perform the asynchronous operation, and then reacquire the lock only to apply a result. A synchronous helper for the snapshot often makes it impossible for the caller to retain the guard accidentally.

The trade-off is that the snapshot may become stale. Depending on the domain, a version check, optimistic retry, or owner-task protocol may be more appropriate than blindly writing the result back. Copying a small request can be clearer and more robust than holding exclusive access while waiting for a remote system.

The decomposition is not a universal transaction guarantee. When concurrent state changes are significant, the design still needs a conflict policy. See [D13](#d13-concurrency-and-shared-state) and [D14](#d14-asynchronous-code).

### B06. Make sensitive diagnostic behavior explicit

A type that contains a secret can deliberately support debugging without exposing its contents. The following wrapper does not attempt secure storage, validation, or memory zeroization; it illustrates only the diagnostic boundary.

```rust
use std::fmt;

/// An opaque token whose debug representation omits its contents.
pub struct AccessToken(String);

impl AccessToken {
    /// Stores a token without validating its contents.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Exposes the token for an operation that explicitly requires it.
    ///
    /// The returned text is sensitive and is not suitable for logging.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessToken")
            .field("value", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_omits_the_token_contents() {
        let token = AccessToken::new("test-only-token".to_owned());
        let output = format!("{token:?}");
        assert!(output.contains("AccessToken"));
        assert!(!output.contains(token.expose_secret()));
    }
}
```

Redacted `Debug` is one layer, not a complete confidentiality mechanism. A caller can still expose the raw value explicitly, and another serialization or tracing path can introduce a leak. See [D25](#d25-diagnostics-and-sensitive-data).

### B07. Review the usage before the implementation

For a new public interface, begin with representative call sites: the ordinary successful case, invalid input, a relevant operational failure, and an ownership or concurrency boundary. This often reveals unnecessary generic parameters, ambiguous booleans, missing error distinctions, or a lifetime that ties unrelated values together.

Then review how the implementation preserves the promised contract. A polished implementation cannot compensate for a contract that is difficult to use correctly. Conversely, implementation complexity can be worthwhile when it is contained behind a small, genuinely simpler interface. See [G02](#g02-optimize-for-the-reader) and [G06](#g06-keep-interfaces-small-and-durable).

### B08. Refactor in changes that can be verified

When improving legacy code, separate mechanical formatting, module movement, API changes, and behavioral corrections where practical. Preserve a useful test baseline before changing a subtle ownership or error path. Explain observable differences rather than describing a change only as “more idiomatic.”

For an optimization, keep the benchmark or workload that justifies it. For a new abstraction, keep an example demonstrating why callers benefit. For an unsafe implementation, keep the safety argument close to the code and update it when the interface evolves. These records make later simplification possible without forcing the next maintainer to reconstruct the original decision.

## Enforcement

This part is normative except where a configuration or command is identified as illustrative. Automation covers only the rules a tool can actually check; passing the gate is necessary but not sufficient for conformance.

### E01. Minimum conformance gate

A repository MUST enforce formatting, compilation, its selected lint policy, relevant tests, and documentation validation. The gate MUST select the intended workspace members and configurations explicitly enough that a new member or feature is not silently omitted.

The following commands are an **illustrative baseline** for a workspace that supports all its normal Cargo target kinds on the CI host. They assume an installed, repository-pinned toolchain with `rustfmt` and Clippy, a current committed lockfile, and any required native dependencies. Repository-specific matrix jobs supplement them.

```sh
set -eu

cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
RUSTDOCFLAGS="${RUSTDOCFLAGS:+$RUSTDOCFLAGS }-D warnings" \
    cargo doc --workspace --no-deps --locked
```

`cargo test --all-targets` and `cargo test --doc` are separate deliberately. Projects with target kinds that cannot run on the CI host MUST define narrower compile and execution jobs, rather than copying this command set and suppressing the resulting failures.[^cargo-test]

Warnings-as-errors SHOULD be a policy of the pinned CI gate, not an unconditional crate-level `#![deny(warnings)]` in a reusable library. A new compiler can introduce warnings independently of a source change. Compatibility jobs on additional toolchains SHOULD classify new warnings separately from failures to compile or execute the promised behavior.[^clippy-usage]

The gate MUST NOT claim to cover all configurations merely because it includes `--all-features`. It MUST also address the documented minimal configuration and important feature interactions. Target triples, panic strategies, `no_std` support, FFI integration, and MSRV validation need their own applicable jobs. Nightly-only analysis tools MAY run in isolated auxiliary jobs; they MUST NOT silently change the stable-build baseline.

### E02. Example workspace policy

The following manifests are a **minimal structural example**, not a requirement to name or arrange a project this way. The example uses one private package in `crates/frame-codec`. Its `rust-version` is an illustrative minimum; the exact development and release compiler is selected separately by the adopting project.

**Workspace `Cargo.toml`:**

```toml
[workspace]
members = ["crates/frame-codec"]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.85"

[workspace.lints.rust]
unsafe_code = "deny"
unsafe_op_in_unsafe_fn = "deny"
unused_must_use = "deny"
missing_docs = "warn"
missing_debug_implementations = "warn"
unexpected_cfgs = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
undocumented_unsafe_blocks = "warn"
missing_safety_doc = "warn"
await_holding_lock = "warn"
dbg_macro = "warn"
```

**Member `crates/frame-codec/Cargo.toml`:**

```toml
[package]
name = "frame-codec"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
publish = false

[lints]
workspace = true
```

The member's inheritance is necessary. The example's default denial of unsafe code can be narrowed only through the reviewed scope policy in [D15](#d15-unsafe-code). A crate with no intended unsafe code can instead forbid it at the crate root. The lint configuration does not certify dependencies or macro expansions as sound.[^cargo-workspaces]

**Workspace `rustfmt.toml`:**

```toml
edition = "2024"
style_edition = "2024"
max_width = 100
hard_tabs = false
newline_style = "Unix"
```

These settings assume that the formatter can parse the entire workspace using Edition 2024. In a mixed-edition workspace, do not force an incorrect parsing edition for every file; use Cargo's per-package edition information and compatible scoped configuration while keeping the intended formatting style edition explicit.[^rustfmt-config]

The repository MUST record an exact approved formatter/compiler toolchain, normally in `rust-toolchain.toml` or equivalent controlled build configuration. This example intentionally does not invent an organization's approved release number. A developer setup procedure MUST install the required components and target support for that chosen version.

An **illustrative MSRV job** for a project whose support policy includes all ordinary target kinds is:

```sh
cargo +1.85.0 check --workspace --all-targets --locked
```

This assumes that toolchain is installed and that the checked-in dependency resolution supports it. Projects whose MSRV promise covers only the public library MUST state that narrower scope and test the relevant library configuration; they MUST NOT imply that the command above was run when only a newer compiler was used.

### E03. Lint policy and suppression

Clippy SHOULD be used as a configurable review aid, not as a replacement for semantic judgment. A repository MAY enable selected pedantic or restriction lints after assessing their fit. It MUST NOT enable the entire `clippy::restriction` group as a blanket quality measure; that group contains deliberately restrictive and sometimes conflicting policies.[^clippy-usage]

A lint suppression MUST name the specific lint and use the smallest practical scope. Its reason MUST explain why the code is intentional or why the lint does not fit the case. Blanket `allow(warnings)` and undifferentiated crate-wide suppression MUST NOT be used to make an otherwise failing conformance gate pass.

Use `#[expect(..., reason = "...")]` when a diagnostic is intentionally expected in the configurations being checked. Use `#[allow(..., reason = "...")]` when a justified exemption is valid even though the diagnostic is not consistently emitted. An unfulfilled expectation is itself diagnosable, so `expect` is not a universal replacement for every conditional `allow`.[^rust-diagnostics]

A suppression MUST NOT conceal a known safety defect. Generated-code exceptions MAY be broader within the generated boundary, but MUST NOT suppress unrelated hand-written code. Existing suppressions SHOULD be reviewed during toolchain and lint-policy updates.

Clippy's MSRV-aware suggestions and configuration are useful, but MUST NOT replace a build on the actual supported MSRV. Tool configuration SHOULD derive its minimum version from a single maintained policy rather than contradicting the Cargo manifest.[^clippy-config]

### E04. Review requirements

A reviewer MUST evaluate the rules relevant to the change, including those that automation cannot enforce. The review SHOULD establish the following evidence without turning every small change into a full-system audit.

| Concern | Evidence appropriate to the change |
| --- | --- |
| Contract | Inputs, outputs, state changes, ownership, and failure behavior are clear. |
| API | Visibility and bounds are necessary; compatibility effects are understood. |
| Resources | Allocation, I/O, lock, thread, and task lifetimes have a defined policy. |
| Failure | Operational errors propagate correctly; partial effects and recovery are specified. |
| Safety | Unsafe obligations are explicit, locally justified, and preserved by the full interface. |
| Validation | Relevant tests, examples, features, targets, and MSRV jobs exercise the claim. |
| Diagnostics | Errors and logs are useful and do not expose sensitive information. |
| Maintenance | Complexity and dependencies have a reason; comments and documentation remain accurate. |

A blocking review comment MUST cite a requirement, identify a correctness or safety issue, or explain a concrete maintainability concern. Advisory improvements SHOULD be identified as such. Reviewers MUST NOT turn a Best Practice into an unstated requirement after an author has already followed the normative standard.

### E05. Exceptions

A departure from a MUST-level rule requires a recorded exception approved by the owner of the applicable policy and the affected code. An exception MUST be narrow, justified, reviewable, and discoverable from the affected code or configuration. It MUST NOT permit undefined behavior, violate an external contract, or waive a mandatory security obligation.

A SHOULD-level departure normally needs a recorded engineering reason rather than a formal exception. The reason MUST address the trade-off relevant to the recommendation; personal preference alone is insufficient when a consistent local choice already exists.

The following is an **exception-record template**, not an approved exception:

```text
Rule: Dxx — the specific requirement being departed from
Scope: exact crates, files, items, targets, or configurations
Reason: the concrete constraint and why conforming alternatives are unsuitable
Consequences: readability, compatibility, safety-review, and maintenance impact
Compensating controls: tests, checks, documentation, or containment measures
Owner: the maintainer accountable for the exception
Approval: the policy owner and affected code owner
Review trigger: an expiry date or a concrete condition requiring reconsideration
Tracking: a durable issue or decision-record identifier
```

A lint `allow` or formatter skip is not automatically an approved exception. Conversely, an exception MUST include the necessary tool configuration so that the recorded policy and the actual gate agree. A permanent platform or interoperability constraint MAY justify a durable exception with a review trigger rather than an arbitrary expiry date.

### E06. Evolution of the standard

Changes to this document MUST distinguish language or tooling facts from organizational choices. A factual correction SHOULD be made promptly with an appropriate primary source. A new normative rule requires a stated problem, a clear scope, a treatment of existing code, and a realistic enforcement or review strategy.

Canonical principles SHOULD change less frequently than implementation decisions. A new lint, language feature, or popular crate does not by itself justify a new requirement. Maintainers SHOULD preserve stable rule identifiers or record a mapping when reorganizing the document.

A revision MUST NOT imply that all historical code has been revalidated. Projects SHOULD record the revision they adopt and review relevant policy changes alongside toolchain and dependency maintenance. The goal is a standard that remains enforceable and understandable, not an ever-growing list of preferences.

## Repository Maintenance

This section describes this documentation repository; it does not add requirements for projects adopting the standard.

### Development environment

Install [Nix](https://nixos.org/download/), [devenv 2.3 or newer](https://devenv.sh/getting-started/), and [direnv](https://direnv.net/docs/installation.html), and enable the [direnv hook for your shell](https://direnv.net/docs/hook.html). From this repository, authorize the checked-in environment once:

```sh
direnv allow
```

The [.envrc](.envrc) loads [devenv.nix](devenv.nix). The committed [devenv.lock](devenv.lock) pins the Nix inputs, and [rust-toolchain.toml](rust-toolchain.toml) pins the development compiler, Clippy, and formatter to Rust 1.98.1. Devenv also supplies Rust 1.85.0 for independent MSRV validation; a separate rustup installation is not required.

Run the checks in the activated environment:

```sh
verify
verify-msrv
verify-links
```

Without automatic direnv activation, use `devenv shell -- verify`, `devenv shell -- verify-msrv`, or `devenv shell -- verify-links`. `devenv test` runs both local toolchain checks. The first environment build downloads its pinned dependencies. Subsequent code checks use an offline, dependency-free generated Cargo workspace; the external-link check requires network access.

### Validation and updates

[scripts/verify.py](scripts/verify.py) parses README Markdown, validates local links and TOML, and extracts the actual E02 manifests and Rust examples into a temporary workspace. It executes the E01 conformance commands verbatim with the committed [validation lockfile](validation/Cargo.lock), runs the examples' tests and additional [boundary tests](validation/regressions.rs), and checks three expected compiler rejections. The explicitly labeled D03 excerpt is formatted but is not compiled because its imports refer to unspecified project code.

The [GitHub Actions workflow](.github/workflows/ci.yml) runs the repository checks on pushes and pull requests. Live external references have a separate check so a remote outage is distinguishable from a code failure.

Edit the examples in README rather than creating independent copies. Review toolchain and dependency changes together with the resulting checks. Use `devenv update` to refresh Nix inputs deliberately and commit the resulting lockfile. Change the MSRV in both E02 and the MSRV toolchain selection in `devenv.nix`, then rerun validation before making a new compatibility claim.

## References

These are primary sources for the organizational model and the language, API, and tooling facts referenced above. Normative project choices remain the choices of this document. Links to rolling documentation can change; an adopting repository should retain the source revision or toolchain documentation relevant to its own support policy.

[^google-overview]: [Google, Go Style — Overview](https://google.github.io/styleguide/go/).

[^google-guide]: [Google, Go Style Guide](https://google.github.io/styleguide/go/guide).

[^google-decisions]: [Google, Go Style Decisions](https://google.github.io/styleguide/go/decisions).

[^google-practices]: [Google, Go Best Practices](https://google.github.io/styleguide/go/best-practices).

[^rust-185]: [Rust Release Team, Announcing Rust 1.85.0 and Rust 2024, February 20, 2025](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/).

[^cargo-manifest]: [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html).

[^cargo-workspaces]: [The Cargo Book, Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

[^cargo-msrv]: [The Cargo Book, Rust Version](https://doc.rust-lang.org/cargo/reference/rust-version.html).

[^rustfmt-config]: [rust-lang/rustfmt, Configuration Options](https://raw.githubusercontent.com/rust-lang/rustfmt/main/Configurations.md).

[^rust-style]: [The Rust Style Guide](https://doc.rust-lang.org/style-guide/).

[^rust-ub]: [The Rust Reference, Behavior Considered Undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).

[^rust-naming]: [Rust RFC 430, Finalizing Naming Conventions](https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html).

[^api-naming]: [Rust API Guidelines, Naming](https://rust-lang.github.io/api-guidelines/naming.html).

[^rust-drop]: [The Rust Reference, Destructors](https://doc.rust-lang.org/reference/destructors.html).

[^std-iterator]: [Rust Standard Library, Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html).

[^rust-operators]: [The Rust Reference, Operator Expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html).

[^std-from]: [Rust Standard Library, From](https://doc.rust-lang.org/std/convert/trait.From.html).

[^std-string]: [Rust Standard Library, String](https://doc.rust-lang.org/std/string/struct.String.html).

[^std-hashmap]: [Rust Standard Library, HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

[^api-interoperability]: [Rust API Guidelines, Interoperability](https://rust-lang.github.io/api-guidelines/interoperability.html).

[^std-arc]: [Rust Standard Library, Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html).

[^std-clone]: [Rust Standard Library, Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html).

[^rust-traits]: [The Rust Reference, Traits — Dyn Compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).

[^std-borrow]: [Rust Standard Library, Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html).

[^std-hash]: [Rust Standard Library, Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html).

[^api-predictability]: [Rust API Guidelines, Predictability](https://rust-lang.github.io/api-guidelines/predictability.html).

[^rust-diagnostics]: [The Rust Reference, Diagnostic Attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html).

[^std-error]: [Rust Standard Library, Error](https://doc.rust-lang.org/std/error/trait.Error.html).

[^std-panic]: [Rust Standard Library, catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html).

[^api-documentation]: [Rust API Guidelines, Documentation](https://rust-lang.github.io/api-guidelines/documentation.html).

[^rustdoc-links]: [The rustdoc Book, Linking to Items by Name](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html).

[^rustdoc-tests]: [The rustdoc Book, Documentation Tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).

[^std-forget]: [Rust Standard Library, mem::forget](https://doc.rust-lang.org/std/mem/fn.forget.html).

[^std-write]: [Rust Standard Library, io::Write](https://doc.rust-lang.org/std/io/trait.Write.html).

[^edition-env]: [The Rust Edition Guide, Newly Unsafe Functions](https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html).

[^tokio-state]: [Tokio, Shared State](https://tokio.rs/tokio/tutorial/shared-state).

[^std-ordering]: [Rust Standard Library, atomic::Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html).

[^std-future]: [Rust Standard Library, Future](https://doc.rust-lang.org/std/future/trait.Future.html).

[^tokio-join]: [Tokio API Documentation, task::JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html).

[^tokio-select]: [Tokio, Select](https://tokio.rs/tokio/tutorial/select).

[^edition-unsafe-op]: [The Rust Edition Guide, unsafe_op_in_unsafe_fn Warning](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html).

[^rust-layout]: [The Rust Reference, Type Layout](https://doc.rust-lang.org/reference/type-layout.html).

[^std-maybeuninit]: [Rust Standard Library, MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html).

[^std-pin]: [Rust Standard Library, pin](https://doc.rust-lang.org/std/pin/index.html).

[^miri]: [rust-lang/miri, Project Documentation](https://github.com/rust-lang/miri).

[^edition-extern]: [The Rust Edition Guide, Unsafe extern Blocks](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html).

[^rust-ffi]: [The Rustonomicon, FFI](https://doc.rust-lang.org/nomicon/ffi.html).

[^edition-unsafe-attrs]: [The Rust Edition Guide, Unsafe Attributes](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html).

[^rust-macros]: [The Rust Reference, Macros by Example](https://doc.rust-lang.org/reference/macros-by-example.html).

[^rust-cfg]: [The Rust Reference, Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html).

[^edition-resolver]: [The Rust Edition Guide, Cargo: Rust-Version Aware Resolver](https://doc.rust-lang.org/edition-guide/rust-2024/cargo-resolver.html).

[^cargo-build]: [The Cargo Book, Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

[^cargo-lock]: [The Cargo Book, Cargo.toml vs Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).

[^cargo-features]: [The Cargo Book, Features](https://doc.rust-lang.org/cargo/reference/features.html).

[^cargo-test]: [The Cargo Book, cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

[^cargo-semver]: [The Cargo Book, SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html).

[^clippy-usage]: [Clippy Documentation, Usage](https://doc.rust-lang.org/clippy/usage.html).

[^clippy-config]: [Clippy Documentation, Configuration](https://doc.rust-lang.org/clippy/configuration.html).

[^rust-style-items]: Rust Project, [The Rust Style Guide: Items](https://doc.rust-lang.org/style-guide/items.html). Item organization and import ordering.

[^std-fromstr]: Rust Project, [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html). Text parsing, lifetime constraints, and representation contracts.
