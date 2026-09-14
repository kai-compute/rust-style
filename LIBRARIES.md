# Rust Libraries

This catalog is a starting point for choosing established Rust dependencies. It favors crates with a clear purpose, a public upstream, stable release practice, useful documentation and tests, and a maintenance history that can be inspected by the consuming project. A listing is a search aid, not a security approval or a substitute for reviewing the exact version and feature set.

Use the standard library when it already provides the required behavior. Add a dependency when it removes meaningful complexity, supplies a capability the standard library does not provide, or gives the project a well-supported interoperability boundary. Keep the dependency's role narrow and record why it is present.

## Catalog key

Each entry has one of these scopes:

| Scope | Meaning |
| --- | --- |
| Foundation | General-purpose building blocks that are reasonable first candidates for a broad class of projects. |
| Specialist | A focused implementation for a particular protocol, format, platform, or security property. |
| Development | A dependency intended for tests, benchmarks, code generation, or other development-only work. |

The links go to the published crate documentation and the upstream source repository. Read both before adoption: the documentation page describes the package interface, while the source repository describes its current policy, supported targets, and development practices.

## Selection process

For every candidate, check the exact package and version that will enter the lockfile:

1. Confirm that the crate solves the actual requirement and that a standard-library API or an existing dependency cannot do so with less surface area.
2. Read the crate's license, supported Rust versions and targets, feature defaults, public API, build scripts, procedural macros, and transitive dependency graph.
3. Check the [RustSec Advisory Database](https://rustsec.org/) and the project's issue and release history. An absence of an advisory is not proof of security.
4. Prefer a crates.io release with a documented upstream over a Git dependency. A Git dependency requires an immutable revision and a recorded reason.
5. Use the repository lockfile and run relevant checks with `--locked`. Review changes to `Cargo.lock` as dependency changes, including transitive changes.
6. Revisit the choice when the crate changes ownership, loses maintenance, raises its MSRV, changes default features, or introduces a new build-time or runtime capability.

Popularity and download counts are useful context but are not selection criteria by themselves. A small, focused crate with a transparent maintenance model can be a better fit than a large framework.

## Foundation

### Data, errors, and observability

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`serde`](https://docs.rs/crate/serde/latest) | Data model serialization and deserialization traits | [serde-rs/serde](https://github.com/serde-rs/serde) |
| [`serde_json`](https://docs.rs/crate/serde_json/latest) | JSON values and serde-backed JSON I/O | [serde-rs/json](https://github.com/serde-rs/json) |
| [`toml`](https://docs.rs/crate/toml/latest) | TOML parsing and serialization | [toml-rs/toml](https://github.com/toml-rs/toml) |
| [`csv`](https://docs.rs/crate/csv/latest) | RFC 4180-style CSV reading and writing | [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) |
| [`thiserror`](https://docs.rs/crate/thiserror/latest) | Typed error definitions for library APIs | [dtolnay/thiserror](https://github.com/dtolnay/thiserror) |
| [`anyhow`](https://docs.rs/crate/anyhow/latest) | Context-rich application error propagation | [dtolnay/anyhow](https://github.com/dtolnay/anyhow) |
| [`tracing`](https://docs.rs/crate/tracing/latest) | Structured, span-based diagnostics | [tokio-rs/tracing](https://github.com/tokio-rs/tracing) |
| [`tracing-subscriber`](https://docs.rs/crate/tracing-subscriber/latest) | Filtering and formatting tracing events | [tokio-rs/tracing](https://github.com/tokio-rs/tracing) |
| [`log`](https://docs.rs/crate/log/latest) | Small logging facade for ecosystem interoperability | [rust-lang/log](https://github.com/rust-lang/log) |

Use `thiserror` at reusable boundaries and `anyhow` at application boundaries. Choose one primary diagnostics facade for a service; bridging both `log` and `tracing` should be deliberate.

### Text, identifiers, and collections

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`regex`](https://docs.rs/crate/regex/latest) | Regular expressions with predictable linear-time matching | [rust-lang/regex](https://github.com/rust-lang/regex) |
| [`url`](https://docs.rs/crate/url/latest) | URL parsing and serialization | [servo/rust-url](https://github.com/servo/rust-url) |
| [`uuid`](https://docs.rs/crate/uuid/latest) | UUID generation, parsing, and formatting | [uuid-rs/uuid](https://github.com/uuid-rs/uuid) |
| [`time`](https://docs.rs/crate/time/latest) | Date, time, duration, and formatting types | [time-rs/time](https://github.com/time-rs/time) |
| [`chrono`](https://docs.rs/crate/chrono/latest) | Date and time APIs used by established integrations | [chronotope/chrono](https://github.com/chronotope/chrono) |
| [`bytes`](https://docs.rs/crate/bytes/latest) | Reference-counted byte buffers for I/O boundaries | [tokio-rs/bytes](https://github.com/tokio-rs/bytes) |
| [`smallvec`](https://docs.rs/crate/smallvec/latest) | Small-vector optimization when allocation behavior matters | [servo/rust-smallvec](https://github.com/servo/rust-smallvec) |
| [`indexmap`](https://docs.rs/crate/indexmap/latest) | Hash maps and sets with deterministic insertion order | [indexmap-rs/indexmap](https://github.com/indexmap-rs/indexmap) |
| [`itertools`](https://docs.rs/crate/itertools/latest) | Additional iterator adapters and tuple utilities | [rust-itertools/itertools](https://github.com/rust-itertools/itertools) |
| [`bitflags`](https://docs.rs/crate/bitflags/latest) | Type-safe bitmask flags | [bitflags/bitflags](https://github.com/bitflags/bitflags) |

Use `std::collections` first. Add an alternative collection only when its ordering, allocation, memory, or API behavior is part of the requirement. Do not use `chrono` and `time` interchangeably without checking their calendar, timezone, and formatting contracts.

### Command-line applications

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`clap`](https://docs.rs/crate/clap/latest) | Typed command-line parsing and help output | [clap-rs/clap](https://github.com/clap-rs/clap) |
| [`clap_complete`](https://docs.rs/crate/clap_complete/latest) | Shell completion generation for clap commands | [clap-rs/clap](https://github.com/clap-rs/clap) |
| [`dialoguer`](https://docs.rs/crate/dialoguer/latest) | Interactive terminal prompts | [console-rs/dialoguer](https://github.com/console-rs/dialoguer) |
| [`indicatif`](https://docs.rs/crate/indicatif/latest) | Progress bars and spinners | [console-rs/indicatif](https://github.com/console-rs/indicatif) |

Keep command parsing separate from application logic. Interactive terminal dependencies should not be pulled into libraries or non-interactive service binaries.

## Specialist

### Async and network services

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`tokio`](https://docs.rs/crate/tokio/latest) | Async runtime, timers, channels, and I/O | [tokio-rs/tokio](https://github.com/tokio-rs/tokio) |
| [`futures`](https://docs.rs/crate/futures/latest) | Runtime-neutral futures traits and combinators | [rust-lang/futures-rs](https://github.com/rust-lang/futures-rs) |
| [`tokio-util`](https://docs.rs/crate/tokio-util/latest) | Tokio codecs, cancellation, framing, and helpers | [tokio-rs/tokio](https://github.com/tokio-rs/tokio) |
| [`reqwest`](https://docs.rs/crate/reqwest/latest) | High-level HTTP client | [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest) |
| [`http`](https://docs.rs/crate/http/latest) | Shared HTTP request, response, header, and URI types | [hyperium/http](https://github.com/hyperium/http) |
| [`hyper`](https://docs.rs/crate/hyper/latest) | Low-level HTTP client and server primitives | [hyperium/hyper](https://github.com/hyperium/hyper) |
| [`tower`](https://docs.rs/crate/tower/latest) | Service, middleware, and backpressure abstractions | [tower-rs/tower](https://github.com/tower-rs/tower) |
| [`tower-http`](https://docs.rs/crate/tower-http/latest) | HTTP-specific Tower middleware | [tower-rs/tower-http](https://github.com/tower-rs/tower-http) |
| [`axum`](https://docs.rs/crate/axum/latest) | Modular HTTP and WebSocket applications | [tokio-rs/axum](https://github.com/tokio-rs/axum) |
| [`tonic`](https://docs.rs/crate/tonic/latest) | gRPC clients and servers | [hyperium/tonic](https://github.com/hyperium/tonic) |
| [`prost`](https://docs.rs/crate/prost/latest) | Protocol Buffers encoding and code generation | [tokio-rs/prost](https://github.com/tokio-rs/prost) |
| [`socket2`](https://docs.rs/crate/socket2/latest) | Portable access to socket options | [rust-lang/socket2](https://github.com/rust-lang/socket2) |
| [`async-trait`](https://docs.rs/crate/async-trait/latest) | Async trait methods where native traits do not fit | [dtolnay/async-trait](https://github.com/dtolnay/async-trait) |

Choose one async runtime at an application boundary. Keep runtime-specific types out of a reusable library unless runtime integration is the library's purpose. Set explicit timeouts, body limits, cancellation behavior, and TLS policy for network clients and servers.

### TLS and cryptographic building blocks

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`rustls`](https://docs.rs/crate/rustls/latest) | TLS 1.2/1.3 implementation in Rust | [rustls/rustls](https://github.com/rustls/rustls) |
| [`rustls-pki-types`](https://docs.rs/crate/rustls-pki-types/latest) | Shared certificate and key representations | [rustls/pki-types](https://github.com/rustls/pki-types) |
| [`webpki-roots`](https://docs.rs/crate/webpki-roots/latest) | Mozilla-derived trust anchors for rustls | [rustls/webpki-roots](https://github.com/rustls/webpki-roots) |
| [`rand`](https://docs.rs/crate/rand/latest) | General-purpose random number generation APIs | [rust-random/rand](https://github.com/rust-random/rand) |
| [`getrandom`](https://docs.rs/crate/getrandom/latest) | Platform-backed operating-system randomness | [rust-random/getrandom](https://github.com/rust-random/getrandom) |
| [`sha2`](https://docs.rs/crate/sha2/latest) | SHA-2 hash functions | [RustCrypto/hashes](https://github.com/RustCrypto/hashes) |
| [`blake3`](https://docs.rs/crate/blake3/latest) | BLAKE3 hashing and streaming | [BLAKE3-team/BLAKE3](https://github.com/BLAKE3-team/BLAKE3) |
| [`hmac`](https://docs.rs/crate/hmac/latest) | HMAC construction over supported hash functions | [RustCrypto/MACs](https://github.com/RustCrypto/MACs) |
| [`hkdf`](https://docs.rs/crate/hkdf/latest) | HKDF key derivation | [RustCrypto/KDFs](https://github.com/RustCrypto/KDFs) |
| [`ed25519-dalek`](https://docs.rs/crate/ed25519-dalek/latest) | Ed25519 signatures and verification | [dalek-cryptography/ed25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek) |
| [`p256`](https://docs.rs/crate/p256/latest) | NIST P-256 elliptic-curve operations | [RustCrypto/elliptic-curves](https://github.com/RustCrypto/elliptic-curves) |
| [`ring`](https://docs.rs/crate/ring/latest) | Low-level cryptographic primitives with a portable API | [briansmith/ring](https://github.com/briansmith/ring) |
| [`zeroize`](https://docs.rs/crate/zeroize/latest) | Explicitly clearing sensitive values from memory | [RustCrypto/utils](https://github.com/RustCrypto/utils) |
| [`subtle`](https://docs.rs/crate/subtle/latest) | Constant-time comparisons and choice types | [dalek-cryptography/subtle](https://github.com/dalek-cryptography/subtle) |

Cryptography requires a threat model and protocol review. These crates provide primitives and integrations; they do not make an incorrect protocol, key lifecycle, random source, or certificate policy safe. Prefer a complete, maintained protocol implementation over assembling primitives in application code.

### Parsing, encoding, and code generation

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`winnow`](https://docs.rs/crate/winnow/latest) | Composable, zero-copy parser combinators | [winnow-rs/winnow](https://github.com/winnow-rs/winnow) |
| [`nom`](https://docs.rs/crate/nom/latest) | Byte and text parser combinators | [rust-bakery/nom](https://github.com/rust-bakery/nom) |
| [`memchr`](https://docs.rs/crate/memchr/latest) | Fast byte and substring search primitives | [BurntSushi/memchr](https://github.com/BurntSushi/memchr) |
| [`aho-corasick`](https://docs.rs/crate/aho-corasick/latest) | Multiple-pattern string matching | [BurntSushi/aho-corasick](https://github.com/BurntSushi/aho-corasick) |
| [`base64`](https://docs.rs/crate/base64/latest) | Standard base64 encoding and decoding | [marshallpierce/rust-base64](https://github.com/marshallpierce/rust-base64) |
| [`syn`](https://docs.rs/crate/syn/latest) | Parsing Rust syntax for procedural macros | [dtolnay/syn](https://github.com/dtolnay/syn) |
| [`quote`](https://docs.rs/crate/quote/latest) | Quasi-quoting Rust token streams | [dtolnay/quote](https://github.com/dtolnay/quote) |
| [`proc-macro2`](https://docs.rs/crate/proc-macro2/latest) | Stable token-stream interface for proc macros | [dtolnay/proc-macro2](https://github.com/dtolnay/proc-macro2) |

Bound parser input and decoded sizes before allocating. Treat procedural macros and build-time code generators as executable build dependencies: review their source, permissions, and transitive graph.

### Concurrency and data structures

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`rayon`](https://docs.rs/crate/rayon/latest) | Data-parallel iterators and scoped parallelism | [rayon-rs/rayon](https://github.com/rayon-rs/rayon) |
| [`crossbeam`](https://docs.rs/crate/crossbeam/latest) | Lock-free and scoped concurrency building blocks | [crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) |
| [`parking_lot`](https://docs.rs/crate/parking_lot/latest) | Efficient synchronization primitives | [Amanieu/parking_lot](https://github.com/Amanieu/parking_lot) |
| [`dashmap`](https://docs.rs/crate/dashmap/latest) | Concurrent map for high-contention access patterns | [xacrimon/dashmap](https://github.com/xacrimon/dashmap) |

Start with `std::sync`, atomics, channels, and scoped threads. Add a concurrency crate only after identifying the contention, fairness, memory, or scheduling property that requires it.

### Databases and persistence

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`sqlx`](https://docs.rs/crate/sqlx/latest) | Async SQL clients with optional compile-time query checking | [launchbadge/sqlx](https://github.com/launchbadge/sqlx) |
| [`diesel`](https://docs.rs/crate/diesel/latest) | Typed SQL query building and database access | [diesel-rs/diesel](https://github.com/diesel-rs/diesel) |

Choose a database library after deciding transaction boundaries, migrations, connection lifecycle, cancellation, and deployment targets. A query builder does not replace authorization, input validation, or migration review.

### Files, processes, and archives

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`tempfile`](https://docs.rs/crate/tempfile/latest) | Secure temporary files and directories | [Stebalien/tempfile](https://github.com/Stebalien/tempfile) |
| [`walkdir`](https://docs.rs/crate/walkdir/latest) | Recursive directory traversal | [BurntSushi/walkdir](https://github.com/BurntSushi/walkdir) |
| [`globset`](https://docs.rs/crate/globset/latest) | Efficient sets of glob patterns | [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/master/crates/globset) |
| [`ignore`](https://docs.rs/crate/ignore/latest) | Gitignore-aware directory traversal | [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) |
| [`which`](https://docs.rs/crate/which/latest) | Locate executables using platform search rules | [harryfei/which-rs](https://github.com/harryfei/which-rs) |
| [`flate2`](https://docs.rs/crate/flate2/latest) | DEFLATE and gzip streams | [rust-lang/flate2-rs](https://github.com/rust-lang/flate2-rs) |
| [`zstd`](https://docs.rs/crate/zstd/latest) | Zstandard compression streams | [gyscos/zstd-rs](https://github.com/gyscos/zstd-rs) |
| [`tar`](https://docs.rs/crate/tar/latest) | Tar archive creation and extraction | [composefs/tar-rs](https://github.com/composefs/tar-rs) |
| [`zip`](https://docs.rs/crate/zip/latest) | ZIP archive creation and extraction | [zip-rs/zip2](https://github.com/zip-rs/zip2) |

Archive extraction must enforce path, size, nesting, and resource limits. Compression and archive crates may invoke native libraries or build scripts through optional features; inspect the enabled feature graph for the target platform.

### Platform, FFI, WebAssembly, and embedded

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`libc`](https://docs.rs/crate/libc/latest) | Portable Rust bindings to C platform APIs | [rust-lang/libc](https://github.com/rust-lang/libc) |
| [`nix`](https://docs.rs/crate/nix/latest) | Safe, feature-gated Unix and Linux APIs | [nix-rust/nix](https://github.com/nix-rust/nix) |
| [`cc`](https://docs.rs/crate/cc/latest) | Compiling small C/C++ components from Cargo builds | [rust-lang/cc-rs](https://github.com/rust-lang/cc-rs) |
| [`bindgen`](https://docs.rs/crate/bindgen/latest) | Generating Rust FFI bindings from headers | [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) |
| [`cbindgen`](https://docs.rs/crate/cbindgen/latest) | Generating C/C++ headers from Rust APIs | [mozilla/cbindgen](https://github.com/mozilla/cbindgen) |
| [`wasm-bindgen`](https://docs.rs/crate/wasm-bindgen/latest) | Rust and JavaScript WebAssembly interoperation | [rustwasm/wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) |
| [`js-sys`](https://docs.rs/crate/js-sys/latest) | Bindings to JavaScript standard objects for Wasm | [rustwasm/wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) |
| [`web-sys`](https://docs.rs/crate/web-sys/latest) | Web platform bindings for Wasm | [rustwasm/wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) |
| [`embedded-hal`](https://docs.rs/crate/embedded-hal/latest) | Portable traits for embedded hardware drivers | [rust-embedded/embedded-hal](https://github.com/rust-embedded/embedded-hal) |

FFI and platform crates expose external contracts; they do not remove the need to review ABI, ownership, lifetimes, error codes, and target assumptions. Keep generated bindings reproducible and review changes to generated output together with the generator and headers.

## Development

| Crate | Use for | Upstream |
| --- | --- | --- |
| [`proptest`](https://docs.rs/crate/proptest/latest) | Property-based and shrinking tests | [proptest-rs/proptest](https://github.com/proptest-rs/proptest) |
| [`criterion`](https://docs.rs/crate/criterion/latest) | Statistical microbenchmarks | [criterion-rs/criterion.rs](https://github.com/criterion-rs/criterion.rs) |
| [`insta`](https://docs.rs/crate/insta/latest) | Reviewable snapshot testing | [mitsuhiko/insta](https://github.com/mitsuhiko/insta) |
| [`assert_cmd`](https://docs.rs/crate/assert_cmd/latest) | Testing command-line programs | [assert-rs/assert_cmd](https://github.com/assert-rs/assert_cmd) |
| [`predicates`](https://docs.rs/crate/predicates/latest) | Composable assertions for tests | [assert-rs/predicates-rs](https://github.com/assert-rs/predicates-rs) |

Development dependencies still execute in CI and on contributor machines. Keep them out of runtime packages, pin the lockfile, and review proc-macro and build-script behavior just as you would for production dependencies.

## Supply-chain guardrails

The following controls make a catalog choice auditable and reduce avoidable supply-chain exposure:

- Commit `Cargo.lock` for applications and first-party workspaces. Use `--locked` in CI and release checks.
- Prefer crates.io packages from a documented upstream. Avoid moving Git branches, unreviewed path dependencies, and dependencies whose source or package contents cannot be explained.
- Review `Cargo.toml`, `Cargo.lock`, enabled features, transitive dependencies, `build.rs`, proc-macro crates, and generated code. Build-time code is part of the trusted computing base.
- Run [cargo-audit](https://github.com/RustSec/cargo-audit) against the lockfile and configure [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) for advisories, licenses, bans, and allowed sources.
- Use [cargo-vet](https://github.com/mozilla/cargo-vet) where the project can maintain an audit trust policy. Treat inherited audits as evidence to inspect, not as a blanket waiver.
- Keep feature sets minimal and test the promised combinations. Disabling a dependency's default features on one edge does not guarantee that another dependency has not re-enabled them.
- Subscribe to upstream security notifications and review owner or maintainer changes. Remove dependencies that no longer have a responsible maintenance path.
- Re-run dependency and license checks during updates. A clean result for one lockfile does not certify every future resolution or every target.

When no catalog entry fits, document the gap and evaluate alternatives before writing a new implementation. New code still needs an owner, tests, security review, and a maintenance plan; avoiding a dependency is not automatically lower risk.

## Related references

- [Cargo Book: Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo Book: Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [RustSec Advisory Database](https://rustsec.org/)
- [Rust Secure Code Working Group](https://github.com/rust-secure-code)
- [Rust Project Security Policy](https://www.rust-lang.org/policies/security)
