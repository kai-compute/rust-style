# Functional Programming in Rust

Companion to [Rust Style](README.md).

**Language baseline:** Rust 2024; minimum supported Rust version (MSRV) 1.85.0.

## Overview

Functional programming in Rust is the discipline of expressing domain behavior as compositions of explicit value transformations, representing alternatives and failures in types, and giving state, evaluation, and external interaction clearly defined boundaries. The objective is an implementation whose behavior follows from its inputs, types, composition rules, and stated operational contracts.

This document develops that discipline in the four-part sequence of *Functional Programming in Scala* by Paul Chiusano and Rúnar Bjarnason: foundations; functional design and combinator libraries; common algebraic structures; and effects and I/O. The book supplies the conceptual progression and vocabulary. The Rust interfaces, ownership policies, numeric rules, examples, and conformance requirements below are independently written engineering decisions, not claims made by the book or an official Rust Project standard. Chapter-specific references distinguish that conceptual basis from references establishing Rust language and library behavior.[^book]

The intended reader understands basic Rust syntax and wants a precise account of functional program design. The document applies to domain libraries and applications, including their test code and effect-handling boundaries. It does not require every module to introduce an abstraction with an algebraic name. A direct function, an enum, a standard-library combinator, or a local loop is an appropriate implementation when its contract is clear.

### Normative language and authority

**MUST** and **MUST NOT** establish requirements of this specification. **SHOULD** and **SHOULD NOT** establish defaults; a departure requires a concrete reason recorded in the implementation or its design documentation. **MAY** permits a choice. Definitions and explicitly stated laws are normative for an interface that adopts the corresponding abstraction. Examples illustrate the contracts immediately surrounding them.

Rust's language semantics and the contracts of the selected library versions take precedence over this document. A mathematical law applies only with its stated domain, equality, and evaluation assumptions. Calling an operation `map`, `combine`, or `and_then` is not evidence that it satisfies a law.

The baseline identifies the edition and API floor targeted by the examples. It is not a recommendation to deploy a particular historical compiler. Rust 2024 was released with Rust 1.85.0.[^edition]

### Reading and example conventions

Each fenced `rust` block is an independent example with its own `main` function and imports. Examples use only the standard library. They are intended to be saved as separate files; they are not successive fragments of a single module. Assertions in `main` specify representative expected behavior. Schematic equations appear in `text` blocks and are not Rust syntax.

The examples are standalone standard-library programs and are intended to be read independently.

### Development environment

Install [Nix and devenv](https://devenv.sh/getting-started/) and enable the [direnv shell hook](https://direnv.net/docs/hook.html). From this repository:

```sh
direnv allow
verify
verify-msrv
verify-links
```

Each Rust listing is a standalone standard-library program, so this repository does not require a Cargo workspace.

### Contents

| Part | Chapters |
| --- | --- |
| [I. Foundations](#part-i) | [1. Meaning and purity](#chapter-1); [2. Functions and ownership](#chapter-2); [3. Functional data structures](#chapter-3); [4. Errors as values](#chapter-4); [5. Strictness and laziness](#chapter-5); [6. Explicit state](#chapter-6) |
| [II. Functional Design and Combinator Libraries](#part-ii) | [7. Parallel computation](#chapter-7); [8. Property-based testing](#chapter-8); [9. Parser combinators](#chapter-9) |
| [III. Common Structures in Functional Design](#part-iii) | [10. Monoids](#chapter-10); [11. Functors and monadic composition](#chapter-11); [12. Applicative validation and traversal](#chapter-12) |
| [IV. Effects and I/O](#part-iv) | [13. External effects](#chapter-13); [14. Local mutation](#chapter-14); [15. Stream processing](#chapter-15) |
| Appendices | [A. Conformance](#conformance); [B. Source correspondence](#source-correspondence); [Terminology](#terminology); [References](#references) |

<a id="part-i"></a>
## Part I. Foundations

<a id="chapter-1"></a>
### 1. What Functional Programming Means

#### 1.1 Values, computations, and observations

A **value** is information described by a type. A **computation** produces a value, performs an interaction, advances state, or combines these activities. A **domain value** represents application information rather than an active capability: a document title, a validated quantity, an input event, or a proposed operation. An open file, a lock guard, and a running task are examples of capabilities whose lifecycle also matters.

A **pure function**, on its declared domain, produces a result determined by its explicit arguments and fixed, immutable captured values, without an externally observable interaction or mutation. Its result may be an ordinary value, an error value, a new state, or a description to be interpreted later. The presence of `Result` does not make a function effectful, and returning a description of an operation does not execute that operation.

A pure interface MUST identify all information that can affect the domain result. Current time, environment variables, filesystem contents, global configuration that can change, and externally supplied randomness MUST enter that interface as values obtained by another component. A reference to a service that obtains these values is an execution capability, not the values themselves. This separation of decisions from obtaining inputs and acting on outputs develops the book's initial treatment of pure functions and effect factoring.[^b01]

#### 1.2 Referential transparency and equality

An expression is **referentially transparent relative to an observation model** when replacing its evaluation with an equivalent already-computed value preserves the observations admitted by that model. Write `x ≃ y` for that equivalence. This specification's default value-level observations include returned values, error variants and payloads, sequence order, and termination on the accepted input domain. External reads and writes, shared-state changes, and resource-lifecycle actions remain observable at execution boundaries.

Representation details such as allocation addresses and spare vector capacity are not part of a value abstraction unless its contract makes them significant. A component that inspects such details MUST include them in its reasoning. This choice of observations is explicit; it is not permission to disregard a difference that the application actually uses. The contextual treatment follows the book's later refinement of referential transparency.[^b14]

Equality also needs a contract. For an ordered collection it normally includes element order. For a set it is membership equality. For a state action it includes both the result and final state. For a parser it includes the parsed result, remaining input, and the chosen error semantics. Two closures are compared by their behavior on relevant inputs, not by their addresses.

#### 1.3 Purity, totality, and defined failure

A **total function on a domain** returns a value of its declared result type for every input in that domain. Domain rejection can be total: `Input -> Result<ValidInput, ValidationError>` gives a defined answer for rejected input. A hidden precondition, an indexing panic, or a nonterminating loop is not a substitute for that answer.

Pure domain operations MUST provide defined behavior for all inputs accepted by their public contract. They SHOULD encode important preconditions in types. Arithmetic failure, malformed input, an empty collection, and a missing mapping MUST be represented deliberately rather than delegated to an incidental panic.

The value-level model assumes sufficient execution resources and a functioning implementation platform. It does not treat arithmetic overflow, an invalid slice boundary, or an omitted case as resource exhaustion. Those are algorithmic obligations. An interpreter that waits on external input has a separate progress and failure contract; a proof about its pure decision function does not establish an I/O deadline.

#### 1.4 A pure decision function

The following function calculates a storage requirement. It neither allocates that storage nor consults a device. Its arithmetic is independent of the build profile.

```rust
// Example 01: A total calculation with explicit arithmetic failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BudgetError {
    Overflow,
}

fn required_bytes(
    records: u64,
    bytes_per_record: u64,
    header_bytes: u64,
) -> Result<u64, BudgetError> {
    records
        .checked_mul(bytes_per_record)
        .and_then(|body| body.checked_add(header_bytes))
        .ok_or(BudgetError::Overflow)
}

fn main() {
    assert_eq!(required_bytes(3, 64, 16), Ok(208));
    assert_eq!(required_bytes(0, 64, 16), Ok(16));
    assert_eq!(required_bytes(u64::MAX, 2, 0), Err(BudgetError::Overflow));
    assert_eq!(required_bytes(1, 1, u64::MAX), Err(BudgetError::Overflow));
}
```

An outer component may use this result to choose whether to allocate, reject a request, or report a limit. None of those policies needs to be embedded in the calculation. Checked arithmetic returns an explicit optional result rather than making overflow behavior depend on ordinary integer-operator checks.[^operators]

#### 1.5 The functional core and the execution boundary

A **functional core** contains domain representations, validation, transformations, transition rules, and planning. An **execution boundary** obtains external inputs, invokes the core, interprets its outputs, and manages resources and failures. Dependency direction SHOULD make it possible to test the core without opening files, starting threads, reading a clock, or constructing a production service client.

The boundary is not an unstructured collection of side effects. It MUST define ordering, partial progress, error translation, and cleanup. Conversely, dependency injection alone does not establish purity: a function that calls an injected writer still performs a write. Returning a write request and passing a writer that executes the request are different designs.

The relevant unit is behavior, not syntax. A function body containing local assignment may expose a pure value transformation. A closure with no `mut` token may consult changing external state. Reviews MUST follow the actual dependencies and observations.

<a id="chapter-2"></a>
### 2. Functions, Composition, and Ownership

#### 2.1 Functions as values

A **higher-order function** accepts a function, returns a function, or both. A **closure** packages executable behavior with captured values. A **combinator** builds a computation or value from other computations or values according to a reusable composition rule. These are ordinary tools for separating a traversal from its element operation, a parser from its result transformation, or a state transition from the next transition.[^b02]

Rust function items and function pointers represent named callable behavior. Closures also capture their lexical environment. Generic callable parameters normally preserve the concrete closure type; `impl Trait` can expose a callable result without naming that type. A boxed callable is useful when an interface intentionally stores different concrete callable representations behind one type.[^closures-book][^closures-reference]

An API SHOULD select its representation from how the callable is stored and invoked. It SHOULD NOT add allocation or dynamic dispatch merely to make a short function pipeline look uniform.

#### 2.2 Callable traits are access contracts

`FnOnce` supports invocation that consumes the callable. It is appropriate when an API makes at most one call and permits captured values to be moved into the result. A caller may also supply a callable with stronger repeated-call capabilities.[^fn-once]

`FnMut` supports repeated invocation through mutable access to the callable. Iterator transformations commonly use it. Any changing captured state that affects results belongs to the computation's state contract, even when that state is small.[^fn-mut]

`Fn` supports invocation through shared access to the callable. It describes an access capability. Purity, determinism, termination, and the behavior of transitively invoked operations remain semantic obligations of a pure API.[^fn]

The callable bound MUST match the invocations actually required. A one-shot continuation SHOULD accept `FnOnce`; a traversal that invokes a callable repeatedly SHOULD use the repeated-call capability it needs. `move` controls capture ownership. The closure's use of its captures determines its callable traits.[^closures-reference]

#### 2.3 Composition and partial application

For functions `f: A -> B` and `g: B -> C`, composition produces a function from `A` to `C`. In this document, `then(f, g)` means apply `f` first and `g` second. **Partial application** fixes an argument or an immutable configuration value and returns a function over the remaining input.

```rust
// Example 02: One-shot composition, a reusable configured function,
// and a borrowed higher-order search.
fn then<A, B, C, F, G>(first: F, second: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C,
{
    move |value| second(first(value))
}

fn prefixed(prefix: String) -> impl Fn(&str) -> String {
    move |suffix| format!("{prefix}{suffix}")
}

fn first_position<T, P>(values: &[T], predicate: P) -> Option<usize>
where
    P: FnMut(&T) -> bool,
{
    values.iter().position(predicate)
}

fn main() {
    let byte_length = then(String::into_bytes, |bytes: Vec<u8>| bytes.len());
    assert_eq!(byte_length(String::from("λ")), 2);

    let label = prefixed(String::from("kind:"));
    assert_eq!(label("record"), "kind:record");
    assert_eq!(label("field"), "kind:field");

    let values = [String::from("north"), String::from("south")];
    assert_eq!(first_position(&values, |s| s.starts_with('s')), Some(1));
    assert_eq!(values[0], "north");
}
```

No element cloning is required for the search. The composed function transfers an owned `String` into an owned byte vector. The configured function owns its fixed prefix and borrows each supplied suffix. Each ownership choice corresponds to a use of the data rather than a stylistic preference.

#### 2.4 Ownership-aware equational reasoning

Ownership determines how a program makes a value available. Borrowing, moving, and cloning are distinct operations. Passing `T` transfers an owned value; passing `&T` supplies shared access; passing `&mut T` supplies exclusive access that can expose updates to the caller. The reachable representation still matters: an owned value can contain shared references, and a shared reference can provide interior-mutability operations.[^expressions][^cell]

A law comparing two computations describes two evaluations with equivalent inputs. It does not instruct the implementation to duplicate a resource, reuse a moved binding, or clone an arbitrary object. Test code MUST construct the corresponding inputs legitimately. For one-shot computations, a factory can construct a fresh equivalent computation for each side of a law.

`Clone` is an explicit duplication operation with a type-specific implementation. A cloned smart pointer can share its referent, and a custom clone operation can execute arbitrary code. A pure generic algorithm that clones values MUST require the relevant cloning behavior to preserve its value contract.[^clone]

An interface SHOULD borrow when it only inspects data and consume when it takes responsibility for a transformation or lifecycle. Introducing `Clone` bounds merely to avoid deciding who owns a value usually obscures the design.

#### 2.5 Types guide implementations

A generic function SHOULD request only operations necessary for its contract. Searching with a predicate needs no ordering bound. Transforming an element needs no equality bound. Moving a value from an input into an output needs no cloning bound. Narrow interfaces make dependencies explicit and reduce the space of plausible implementation mistakes.

Parametricity is useful as a design discipline: treat a type parameter uniformly through the operations explicitly provided for it. A proof based on that uniform treatment MUST state its semantic assumptions, including the behavior of supplied callbacks and trait methods. The existence of a generic type parameter alone is not a proof of a behavioral law.

#### 2.6 Expressions, iteration, and clarity

`match` and `if` expressions SHOULD be used when they expose the alternatives of a value directly. Early `return` and `?` SHOULD be used when they make a failure path more apparent than additional nesting. Neither form changes whether the operation is functional.[^expressions][^operators]

Input-dependent repetition SHOULD use iterator consumers or explicit loops with a clearly identified accumulator. A recursive algorithm MUST have an appropriate depth bound or an implementation with bounded call-stack use. The same review applies to constructing and destroying recursive representations.

A readable loop that implements a pure state transition is preferable to a combinator chain that hides ownership, invents intermediate allocations, or obscures termination. Function composition is a means of expressing a contract, not a requirement to compress a program into one expression.

<a id="chapter-3"></a>
### 3. Functional Data Structures

#### 3.1 Products, sums, and invariants

A **product type** groups values that exist together. Rust tuples and structs express products. A **sum type** selects one of several alternatives. Rust enums express sums, with each variant optionally carrying its own product of fields. An **algebraic data type** is constructed from these alternatives and combinations.[^b03][^enums]

A domain representation MUST distinguish semantically different cases. An operation that is either pending, successful, or rejected SHOULD use variants that carry the information appropriate to each case. Unrelated booleans and optional fields SHOULD NOT encode a state space containing combinations the domain rejects.

An invariant that is not implied by a type's shape SHOULD be established by a checked constructor. Fields whose unrestricted mutation would invalidate the constructor's guarantee SHOULD remain private. Pattern matching SHOULD handle the meaningful cases explicitly; a wildcard is appropriate only when all omitted cases genuinely share the same behavior.

#### 3.2 Immutable observations and persistence

A **functional data structure** is operated on through value-preserving observations and transformations that do not change an existing logical value. A **persistent operation** produces a new version while previously available versions retain their meaning. **Structural sharing** reuses unchanged representation nodes across versions. These definitions concern observable versions, not the absence of every internal memory write.[^b03]

A standard `Vec<T>` can be the representation of a value-oriented interface. Consuming a vector, changing its exclusively owned storage, and returning it may implement a pure transformation. Preserving the old vector as a separately usable version is a different requirement, with a corresponding representation or copying cost.

For a persistent interface, an update MUST preserve every existing version's logical contents. Sharing MUST NOT expose a path that lets a client mutate those shared logical contents. This obligation includes the behavior of element types, not just collection links.

#### 3.3 A structurally shared sequence

The following persistent stack keeps representation links private. Cloning a stack clones its root handle, not its elements. Prepending allocates one node and shares the prior tail. The custom destructor walks a uniquely owned suffix iteratively and stops when it reaches a shared node.

```rust
// Example 03: Persistent versions with shared tails.
use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

struct PersistentStack<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> Clone for PersistentStack<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
        }
    }
}

impl<T> PersistentStack<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    fn first(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.value)
    }
}

impl<T> Drop for PersistentStack<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            match Rc::try_unwrap(node) {
                Ok(mut owned) => current = owned.next.take(),
                Err(_) => break,
            }
        }
    }
}

fn main() {
    let base = PersistentStack::new().prepend(10).prepend(20);
    let snapshot = base.clone();
    let extended = base.prepend(30);

    assert_eq!(base.iter().copied().collect::<Vec<_>>(), vec![20, 10]);
    assert_eq!(snapshot.iter().copied().collect::<Vec<_>>(), vec![20, 10]);
    assert_eq!(
        extended.iter().copied().collect::<Vec<_>>(),
        vec![30, 20, 10]
    );
    assert_eq!(extended.tail().first(), Some(&20));
    assert_eq!(PersistentStack::<u8>::new().first(), None);
}
```

`Rc` provides single-threaded reference-counted ownership; its cloning and `try_unwrap` operations supply the mechanisms used here.[^rc] The stack's persistence claim assumes that an element's logical value is stable through the observations the API provides. For example, changing an element through interior mutability would change what multiple versions observe, even though their links are unchanged.

The principal laws are:

```text
first(empty)                  = None
tail(empty)                   ≃ empty
first(prepend(x, xs))          = Some(x)
tail(prepend(x, xs))           ≃ xs
elements(clone_version(xs))    = elements(xs)
```

Here `Some(x)` describes the observed element, not a requirement to move it out of a borrowed stack. Root cloning, prepending, selecting the first element, and selecting the tail take constant structural work. Traversal takes work proportional to the visited nodes. Node reclamation takes work proportional to the uniquely owned suffix released. Element construction, observation, and destruction have their own costs and contracts.

#### 3.4 Folds and structure

A **fold** summarizes a structure by replacing its construction steps with combining operations. For a finite sequence and an accumulator `z`, a left fold applies the step in encounter order. A right fold nests the element operation from the other end. Neither operation requires associativity when its specified order is preserved.[^b03]

```text
fold_left([a, b, c], z, step)  = step(step(step(z, a), b), c)
fold_right([a, b, c], z, step) = step(a, step(b, step(c, z)))
```

`Iterator::fold` accepts an accumulator operation. A double-ended iterator also supports `rfold`; its closure receives the accumulator first, so an element-first right-fold equation requires the corresponding argument arrangement.[^iterator][^double-ended]

```rust
// Example 04: Fold direction is part of the result contract.
fn main() {
    let values = [1_i32, 2, 3];
    let left = values.iter().copied().fold(0, |acc, x| acc - x);
    let right = values.iter().copied().rfold(0, |acc, x| x - acc);

    assert_eq!(left, -6);
    assert_eq!(right, 2);

    let text = ["a", "b", "c"]
        .into_iter()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        });
    assert_eq!(text, "abc");
}
```

A tree fold has one combining case for each constructor. Its traversal order MUST be specified when callbacks or diagnostics can distinguish that order. An implementation SHOULD use an explicit work stack for input-dependent depth and SHOULD account for the work stack in its space bound.

#### 3.5 Mapping, filtering, and flattening

**Mapping** changes element values while preserving the relevant structure. **Filtering** selects elements satisfying a predicate. **Flattening** joins one layer of nested sequence structure. A sequence `flat_map` maps each element to a sequence and concatenates those sequences in the chosen encounter order.

These operations have different structural contracts. A map over a sequence preserves its length and positions. A filter preserves the relative order of retained elements but may change length. Flattening preserves the specified outer and inner orders. A function that sorts or deduplicates as part of a nominal `map` violates the ordinary sequence-mapping contract.

The implementation MUST specify whether it consumes elements, borrows them, or produces owned copies. Borrowed iteration and owned iteration can express the same value transformation with different ownership and lifetime contracts.[^iter-module]

#### 3.6 Copy-on-write and representation selection

A copy-on-write result is useful when a transformation frequently returns the original text unchanged. The following function has deliberately ASCII-specific semantics: only ASCII uppercase letters are converted.

```rust
// Example 05: Preserve a borrow when no transformation is necessary.
use std::borrow::Cow;

fn ascii_lowercase(input: &str) -> Cow<'_, str> {
    if input.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(input.to_ascii_lowercase())
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    assert!(matches!(ascii_lowercase("record"), Cow::Borrowed(_)));
    assert!(matches!(ascii_lowercase("Record"), Cow::Owned(_)));
    assert_eq!(ascii_lowercase("Record-Δ"), "record-Δ");
}
```

`Cow` represents either borrowed data or owned data. Its ownership variants are representation choices; a transformation's semantic claim must still be stated independently.[^cow] A collection SHOULD be chosen for its actual operations, locality, versioning needs, and ownership costs. Structural sharing is useful when retaining versions is useful. A contiguous owned buffer is useful when a single owner performs a linear transformation.

<a id="chapter-4"></a>
### 4. Handling Errors as Values

#### 4.1 Absence, rejection, and unsuccessful interaction

An error value represents a defined unsuccessful outcome. It lets a caller compose operations before choosing a recovery, reporting, or termination policy. The book develops this approach through optional values and a success-or-error sum type; in Rust, the corresponding everyday interfaces are `Option<T>` and `Result<T, E>`.[^b04]

`Option<T>` SHOULD represent meaningful absence when a separate explanation is unnecessary. `Result<T, E>` SHOULD represent rejection or failure whose cause belongs in the contract. `Result<Option<T>, E>` distinguishes an unsuccessful operation from a successful operation that found nothing. `Option<Result<T, E>>` distinguishes an absent attempted computation from a present success or failure. The nesting is information, not incidental punctuation.[^option][^result]

A library MUST NOT encode ordinary failure using an arbitrary successful value, an unexplained empty collection, a fabricated default, or an unchecked extraction. Converting an error into absence or a default is a policy decision and MUST be explicit at the layer that owns that policy.

#### 4.2 Validated construction

The accepted textual grammar of a constructor MUST be stated. Parsing a number, normalizing whitespace, allowing a leading sign, and validating a nonzero range are separate decisions. A constructor SHOULD expose the distinctions callers actually need without leaking an irrelevant implementation detail.

```rust
// Example 06: An explicit grammar followed by a domain invariant.
use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LimitError {
    InvalidDigits,
    OutOfRange,
    Zero,
}

fn parse_limit(text: &str) -> Result<NonZeroU16, LimitError> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(LimitError::InvalidDigits);
    }
    let value = text.parse::<u16>().map_err(|_| LimitError::OutOfRange)?;
    NonZeroU16::new(value).ok_or(LimitError::Zero)
}

fn main() {
    assert_eq!(parse_limit("12").map(NonZeroU16::get), Ok(12));
    assert_eq!(parse_limit("00012").map(NonZeroU16::get), Ok(12));
    assert_eq!(parse_limit("0"), Err(LimitError::Zero));
    assert_eq!(parse_limit("65536"), Err(LimitError::OutOfRange));
    assert_eq!(parse_limit("+12"), Err(LimitError::InvalidDigits));
    assert_eq!(parse_limit(" 12"), Err(LimitError::InvalidDigits));
    assert_eq!(parse_limit(""), Err(LimitError::InvalidDigits));
}
```

This interface accepts one or more ASCII decimal digits, including leading zeroes, and then requires a value in `1..=65535`. The nonzero type carries the established invariant beyond parsing.[^nonzero]

A domain error SHOULD be a structured type when consumers need to branch on its cause. Human-readable messages SHOULD be produced from that structure. Public errors intended to participate in ordinary Rust error reporting SHOULD implement `Display` and `std::error::Error`, preserving a source error where that source is useful.[^error]

#### 4.3 Transforming and sequencing results

`map` transforms a successful value while preserving absence or failure. `and_then` sequences a function that itself returns an optional or fallible value. `map_err` transforms the error channel. `or_else` introduces a recovery computation. Each operation SHOULD be selected for the information flow it expresses.[^option][^result]

The `?` operator is appropriate for readable dependent sequencing. On `Result`, it continues with the success value or returns an error from the enclosing function or closure, applying the relevant error conversion. On `Option`, it continues with a present value or returns `None`. The return type establishes which of these behaviors the enclosing computation uses.[^operators]

A function SHOULD NOT construct several levels of nested closures merely to avoid `?`. Conversely, a short `and_then` or `map` often communicates a local transformation more directly than a complete `match`. The choice is about exposing the dependency structure.

#### 4.4 Sequencing collections and transposing layers

**Sequencing** changes a collection of contextual values into a contextual collection. **Traversal** applies a contextual operation to each element and sequences its results. For an ordered iterator of `Result` values, collecting into `Result<Vec<_>, _>` returns the values in encounter order when all succeed and stops at the first encountered error. The analogous optional collection stops at absence.[^result-collection][^option-collection]

```rust
// Example 07: Optional parsing, fail-fast collection, and error order.
use std::num::ParseIntError;

fn parse_optional(input: Option<&str>) -> Result<Option<u32>, ParseIntError> {
    input.map(str::parse::<u32>).transpose()
}

fn main() {
    assert_eq!(parse_optional(None).unwrap(), None);
    assert_eq!(parse_optional(Some("17")).unwrap(), Some(17));
    assert!(parse_optional(Some("invalid")).is_err());

    let mut visited = Vec::new();
    let parsed: Result<Vec<u32>, _> = ["4", "invalid", "9"]
        .into_iter()
        .map(|text| {
            visited.push(text);
            text.parse::<u32>()
        })
        .collect();

    assert!(parsed.is_err());
    assert_eq!(visited, vec!["4", "invalid"]);

    let present: Option<Vec<u32>> = [Some(2), Some(3)].into_iter().collect();
    let absent: Option<Vec<u32>> = [Some(2), None, Some(3)].into_iter().collect();
    assert_eq!(present, Some(vec![2, 3]));
    assert_eq!(absent, None);
}
```

The visitation vector is test instrumentation that makes short-circuiting visible. The optional parsing function in this example intentionally delegates its numeric syntax to the standard `u32` parser. `transpose` exchanges optional and fallible layers while preserving their cases; it does not perform recovery.[^option]

A fallible input stream MUST NOT be treated as a stream of successful values by silently discarding its errors. In particular, `filter_map(Result::ok)` and flattening a sequence of results make omission a policy. They are appropriate only when the API explicitly specifies that failures are discarded and explains why that is correct.

#### 4.5 Fail-fast behavior versus accumulated validation

Dependent operations SHOULD use fail-fast sequencing: a later operation needs an earlier successful value. Independent validations SHOULD accumulate errors when the caller needs a complete account of the rejected input. Accumulation requires an explicit error combination rule and is developed in [Chapter 12](#chapter-12).

Returning `Result<T, Vec<E>>` does not itself establish accumulation. The implementation must actually evaluate the selected validations and combine their failures. Likewise, calling two validation functions before passing their already-computed results to a combinator is different from deferring the second function until the first succeeds.

Error order MUST be deterministic when it is part of the public result. A refactoring that changes which error is returned first can be a behavioral change even when every successful result remains the same.

#### 4.6 Panics and recovery policy

Expected rejection MUST use a value-level error path. Assertions SHOULD establish internal invariants or test expectations, rather than validate untrusted input in the functional core. `unwrap` and `expect` in an example's assertions establish a test fixture's expected success; they do not prescribe a production error-handling policy.

A recovery MUST be narrow enough to preserve the meaning of the error. A parse failure is not evidence that an input should be treated as zero. A failed read is not evidence that the source is empty. A rejected state transition is not evidence that execution of a corresponding external operation succeeded.

Recovery that retries an external operation belongs to the execution boundary. Its repeated effects, attempt limits, and partial-progress behavior MUST be specified separately from the pure function that decides whether another attempt is appropriate.

<a id="chapter-5"></a>
### 5. Strictness, Laziness, and Demand

#### 5.1 Evaluation is part of a combinator's contract

A strict argument is evaluated before the called operation uses its value. A deferred computation packages the work in a callable or another description that a consumer evaluates later. The book uses this distinction to separate what a computation describes from how much of it is evaluated.[^b05]

Ordinary Rust call arguments are evaluated before the call. Passing a closure delays its body, not work already done to construct its captured values. An API whose fallback must be conditional SHOULD accept a callable for that fallback. For example, `unwrap_or_else` defers the fallback body, whereas a value supplied to `unwrap_or` has already been evaluated.[^expressions][^closures-book]

```rust
// Example 08: Evaluation of an unused fallback is omitted.
fn choose<T, Y, N>(condition: bool, yes: Y, no: N) -> T
where
    Y: FnOnce() -> T,
    N: FnOnce() -> T,
{
    if condition { yes() } else { no() }
}

fn main() {
    let mut fallback_calls = 0;
    let result = Some(String::from("configured")).unwrap_or_else(|| {
        fallback_calls += 1;
        String::from("default")
    });
    assert_eq!(result, "configured");
    assert_eq!(fallback_calls, 0);

    let selected = choose(false, || String::from("left"), || String::from("right"));
    assert_eq!(selected, "right");
}
```

A skipped closure body does not imply that its captured values have no lifecycle. Captures are still constructed, moved, and eventually dropped. Resource-bearing captures therefore need an ownership contract in addition to an invocation contract.

#### 5.2 Lazy iterator transformations

An iterator is an advancing cursor over a sequence of items. Adapters such as `map` and `filter` describe transformations performed as items are requested. Consumers such as `collect`, `fold`, and `find` drive that process. A lazy pipeline can process successive elements without materializing a complete intermediate collection for every adapter.[^iterators-book]

The iterator cursor is operational state. A function that constructs, consumes, and discards a cursor locally can nevertheless implement a pure transformation over its input sequence. Purity requires the source and callbacks to satisfy the function's semantic contract; iterator syntax does not remove external interactions from a source.

A pipeline SHOULD keep transformations lazy until a concrete collection, repeated traversal, or an ownership boundary is required. It MUST NOT collect an intermediate result solely to call another operation that can work directly on the iterator, unless the materialization has a stated purpose.

#### 5.3 Short-circuiting and input consumption

The demand contract MUST distinguish emitted values from consumed input. A consumer that needs ten matching values may inspect many more than ten inputs. Filtering a source and then applying `take(10)` therefore establishes an output bound, not an input-work bound.

Operations that search, test a prefix, or combine sources have their own consumption behavior. `take_while` consumes the first item that fails its predicate. A peek can request an upstream item while retaining it locally. Zipping ends when one side ends and can advance one side while discovering exhaustion of the other. Code that relies on exact consumption MUST use the documented adapter contracts and test the relevant boundary cases.[^iterator]

Potentially unbounded sources MUST be paired with a justified termination argument or a bounded operational policy. A parser, filter, or consumer that can continue scanning without producing a result needs an input, time, or work budget where the application requires one.

#### 5.4 Unfolding explicit state

An **unfold** generates a sequence from a state transition that either ends production or returns an item and the next state. Unlike a fold, which summarizes input, an unfold constructs output. Its conceptual form is:

```text
step: S -> Option<(A, S)>
unfold(s, step): sequence of A
```

Each successful step MUST define both the emitted item and the state used for the following step. Each requested item must be obtainable in finite work on the supported domain. This is the relevant productivity requirement.[^b05]

```rust
// Example 09: An owned-state unfold with permanent exhaustion.
use std::iter::{self, FusedIterator};

fn unfold<S, A, F>(initial: S, mut step: F) -> impl Iterator<Item = A> + FusedIterator
where
    F: FnMut(S) -> Option<(A, S)>,
{
    let mut state = Some(initial);
    iter::from_fn(move || {
        let current = state.take()?;
        let (item, next) = step(current)?;
        state = Some(next);
        Some(item)
    })
    .fuse()
}

fn main() {
    let mut countdown = unfold(3_u8, |remaining| match remaining {
        0 => None,
        n => Some((n, n - 1)),
    });
    assert_eq!(countdown.by_ref().collect::<Vec<_>>(), vec![3, 2, 1]);
    assert_eq!(countdown.next(), None);
    assert_eq!(countdown.next(), None);

    let powers = iter::successors(Some(1_u64), |n| n.checked_mul(2));
    assert_eq!(powers.count(), 64);
}
```

`from_fn` constructs an iterator from a stateful next-item closure. `successors` handles the common case in which each item determines the following optional item. A fused iterator promises permanent exhaustion after its first `None`; that promise must be established by the representation or an appropriate adapter.[^from-fn][^successors][^fused]

#### 5.5 Laziness and memoization are different contracts

**Laziness** postpones work. **Memoization** retains a computed result for later reuse. A deferred callable may recompute on each invocation; an iterator advances; a cache preserves selected results. An API MUST identify which of these behaviors it provides.

A memoized pure result MUST be determined by a complete immutable key or immutable owning value. Initialization MUST avoid reentering the same uninitialized cell. Cache contents, capacity, and hit counts SHOULD remain outside the domain interface unless their observability is intentional. `OnceCell` provides a single-assignment cell whose initialized value can be accessed through shared references.[^once-cell]

```rust
// Example 10: Memoization behind an immutable value interface.
use std::cell::OnceCell;

struct Label {
    raw: String,
    normalized: OnceCell<String>,
}

impl Label {
    fn new(raw: String) -> Self {
        Self {
            raw,
            normalized: OnceCell::new(),
        }
    }

    fn normalized(&self) -> &str {
        self.normalized
            .get_or_init(|| self.raw.trim().to_ascii_lowercase())
    }
}

fn main() {
    let label = Label::new(String::from("  RECORD  "));
    assert_eq!(label.normalized(), "record");
    assert_eq!(label.normalized(), "record");
}
```

This label trims the standard string whitespace set and lowercases ASCII letters. Its raw text is fixed after construction. The cached representation does not change its normalized logical value. The same argument would need reevaluation if a mutator for the raw field were added.

#### 5.6 Fusion and retained memory

For pure callbacks and equivalent source observations, mapping twice can be reasoned about as mapping with a composed function. The law describes values and demand; it does not promise a particular optimizer decision or machine-code shape.

```text
map(map(xs, f), g) ≃ map(xs, x -> g(f(x)))
```

Removing a materialization point can change when callbacks run and when values are released. Such a change MUST preserve any relevant error precedence, resource lifetime, and externally visible order. A value-level equation alone is insufficient when those observations are present.

Space claims MUST account for retained inputs, captured values, memoized results, output collections, and buffering. An adapter with a small internal state can still keep a large source alive. A cache can make repeated work cheaper while retaining every computed result. The intended retention policy belongs in the interface's operational documentation.

<a id="chapter-6"></a>
### 6. Purely Functional State

#### 6.1 State is an input and an output

A **state action** is a transformation that consumes a state and produces a result together with a subsequent state:

```text
State action: S -> (A, S)
```

The state is not an invisible global variable. It is part of the computation's data flow. The book develops this representation using pseudorandom generation and generalizes it to state machines.[^b06]

The subsequent action MUST receive the state returned by the preceding action when sequential behavior is intended. Reusing an earlier state deliberately replays or branches the computation. It does not advance the same sequence.

Explicit state does not prescribe an allocation strategy. A small state may be copied. An owned collection may be moved and updated locally. A persistent state may share unchanged data. The semantic obligation is to preserve the declared relationship between input state, output value, and output state.

#### 6.2 Composing state actions

Mapping a state action transforms its result and preserves its resulting state. Binding a state action chooses a following action from the first result and threads the resulting state into it. A constant action returns a value without changing the state.

```rust
// Example 11: One-shot state composition without cloning queue elements.
use std::collections::VecDeque;

fn state_pure<S, A>(value: A) -> impl FnOnce(S) -> (A, S) {
    move |state| (value, state)
}

fn map_state<S, A, B, P, F>(program: P, transform: F) -> impl FnOnce(S) -> (B, S)
where
    P: FnOnce(S) -> (A, S),
    F: FnOnce(A) -> B,
{
    move |state| {
        let (value, next_state) = program(state);
        (transform(value), next_state)
    }
}

fn and_then_state<S, A, B, P, F, Q>(program: P, next: F) -> impl FnOnce(S) -> (B, S)
where
    P: FnOnce(S) -> (A, S),
    F: FnOnce(A) -> Q,
    Q: FnOnce(S) -> (B, S),
{
    move |state| {
        let (value, next_state) = program(state);
        next(value)(next_state)
    }
}

fn pop_front(mut queue: VecDeque<String>) -> (Option<String>, VecDeque<String>) {
    let value = queue.pop_front();
    (value, queue)
}

fn main() {
    let queue: VecDeque<String> = ["alpha", "beta", "gamma"]
        .into_iter()
        .map(String::from)
        .collect();

    let first_two = and_then_state(pop_front, |first| {
        map_state(pop_front, move |second| (first, second))
    });
    let ((first, second), remaining) = first_two(queue);
    assert_eq!(first.as_deref(), Some("alpha"));
    assert_eq!(second.as_deref(), Some("beta"));
    assert_eq!(remaining.front().map(String::as_str), Some("gamma"));

    let (value, state) = state_pure(String::from("fixed"))(7_u8);
    assert_eq!(value, "fixed");
    assert_eq!(state, 7);
}
```

The helpers package one-shot programs. Their construction does not invoke the state actions. Purity requires that the actions, result transformations, captures, and element lifecycle satisfy the value contract. Using `FnOnce` permits results and captured values to move rather than requiring duplication.

For a small, fixed sequence, directly binding `(value, next_state)` pairs is often clearer than constructing a composed action. The helpers become useful when a program must be assembled, passed to another component, or combined repeatedly according to the same pattern.

#### 6.3 Observational equality and state laws

Two state actions are equivalent when, for every relevant initial state, they return equivalent result values and equivalent final states. Comparing only their first tuple components is insufficient.

```text
run(state_pure(a), s)                = (a, s)
run(map_state(p, f), s)              = let (a, s1) = run(p, s) in (f(a), s1)
run(and_then_state(p, next), s)      = let (a, s1) = run(p, s) in run(next(a), s1)
```

These equations determine which state is threaded. Reordering independent-looking actions is valid only when their complete state transformations commute under the chosen observations. Sharing the same state type is not evidence of independence.

Tests for replay SHOULD construct equivalent initial states and programs independently. Tests for sequential execution SHOULD verify that the second action receives the first action's resulting state. Tests for branching SHOULD verify that the intended branch point is retained rather than accidentally overwritten.

#### 6.4 Replayable generation

A deterministic pseudorandom generator can be modeled as `Seed -> (Sample, Seed)`. Reproduction requires the initial state, the generator algorithm and configuration, and the same sequence of sampling decisions. Obtaining an unpredictable seed is an external interaction; transforming an explicit seed can be a pure operation.[^b06]

A test harness SHOULD retain enough information to replay the actual failing input, not just a timestamp. Reusing a seed produces the same deterministic sequence. Duplicating a seed does not establish statistical independence between computations. An application that requires a particular distribution or statistical quality MUST establish that requirement separately from replayability.

#### 6.5 State and failure

The placement of `Result` determines what remains available on failure. The following are different contracts:

```text
S -> Result<(A, S), E>        Success returns a new state; failure returns only E.
S -> (Result<A, E>, S)        Both success and failure return a state.
S -> Result<(A, S), (E, S)>   Each outcome explicitly carries its associated state.
```

An API MUST define whether failure retains the original state, returns a partially advanced state, or consumes state without returning it. A fallible transformation that promises rejection without a state change MUST construct or validate its next state before publishing it. The type alone does not establish rollback of external interactions or changes through shared aliases.

#### 6.6 A checked transition system

A state machine SHOULD expose a transition function over domain events, with invariants established by construction. The following machine accepts increments up to a limit and a reset event. Rejection returns the unchanged state.

```rust
// Example 12: A state transition with explicit rejection-state semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Counter {
    value: u32,
    limit: u32,
}

impl Counter {
    fn new(limit: u32) -> Self {
        Self { value: 0, limit }
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    Add(u32),
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionError {
    Overflow,
    LimitExceeded,
}

fn transition(state: Counter, event: Event) -> Result<(u32, Counter), (TransitionError, Counter)> {
    let next_value = match event {
        Event::Reset => 0,
        Event::Add(amount) => match state.value.checked_add(amount) {
            Some(value) => value,
            None => return Err((TransitionError::Overflow, state)),
        },
    };
    if next_value > state.limit {
        return Err((TransitionError::LimitExceeded, state));
    }
    let next = Counter {
        value: next_value,
        ..state
    };
    Ok((next_value, next))
}

fn main() {
    let initial = Counter::new(10);
    let (value, advanced) = transition(initial, Event::Add(7)).unwrap();
    assert_eq!(value, 7);
    assert_eq!(initial.value, 0);
    assert_eq!(
        transition(advanced, Event::Add(4)),
        Err((TransitionError::LimitExceeded, advanced)),
    );
    assert_eq!(transition(advanced, Event::Reset), Ok((0, initial)));

    let (_, full) = transition(Counter::new(u32::MAX), Event::Add(u32::MAX)).unwrap();
    assert_eq!(
        transition(full, Event::Add(1)),
        Err((TransitionError::Overflow, full)),
    );
}
```

Every successful transition preserves `value <= limit`. Addition by zero is a defined identity event. Reset restores the zero state for the same limit. These statements are properties of the transition function, independent of where events originate or how state is persisted.

An execution layer may obtain an event, call the transition, and persist the accepted next state. Persistence failure, concurrent updates, and retry policy belong to that execution layer. Keeping the transition pure makes the decision replayable; it does not turn a sequence of external writes into a transaction.

<a id="part-ii"></a>
## Part II. Functional Design and Combinator Libraries

Functional library design begins with concrete use cases and the information needed to express them. It then identifies representations, a small set of primitives, the laws governing those primitives, and operations derived from them. Implementation experiments and adversarial examples refine the contracts. This is the design framing of the book's second part; the following chapters apply it to Rust representations.[^b07][^b08][^b09]

An abstraction SHOULD be introduced when it makes several operations expressible through one coherent contract. Its public surface MUST specify results, failure, evaluation, and resource ownership before implementation details become accidental policy.

<a id="chapter-7"></a>
### 7. Functional Parallel Computation

#### 7.1 Separate the work from its execution

A **parallel computation** consists of work units whose execution may overlap. A functional design separates the values and functions describing that work from the mechanism that starts it, schedules it, waits for it, and handles execution failure. This separation is central to the book's treatment of parallelism.[^b07]

A task description SHOULD capture immutable input values or suitable borrows and a deterministic computation. Constructing the description MUST NOT silently execute the task when the API promises deferred execution. Starting a thread is an execution action, even when the thread's body computes a pure result.

The pure meaning of two independent tasks is a pair of results. Their executor additionally has operational outcomes such as failure to start a worker. A claim that sequential and parallel executions agree MUST state whether it concerns successful domain results or also includes these execution outcomes.

#### 7.2 Ownership and task boundaries

Data sent to another thread must satisfy the relevant `Send` contract. Shared references sent across threads require appropriate `Sync` behavior of their referents. These traits concern thread transfer and sharing; the task's functional contract additionally requires stable logical inputs and deterministic operations.[^send][^sync]

Scoped threads can borrow data whose lifetime covers the scope. Their job bodies are joined before the scope returns. An explicitly joined worker exposes its panic through the join result rather than leaving the scope to handle an automatically joined panic.[^thread-scope][^scoped-join]

The following executor starts one worker for the left task and performs the right task in the calling thread. Consequently, only the left task and its result require `Send`. Thread-start failure is returned as an execution error.

```rust
// Example 13: Execute a pair of independent tasks with bounded spawning.
use std::{io, thread};

#[derive(Debug)]
enum ParallelError {
    Start(io::Error),
    WorkerPanicked,
}

fn parallel_pair<A, B, F, G>(left: F, right: G) -> Result<(A, B), ParallelError>
where
    F: FnOnce() -> A + Send,
    G: FnOnce() -> B,
    A: Send,
{
    thread::scope(|scope| {
        let worker = thread::Builder::new()
            .spawn_scoped(scope, left)
            .map_err(ParallelError::Start)?;
        let right_value = right();
        let left_value = worker.join().map_err(|_| ParallelError::WorkerPanicked)?;
        Ok((left_value, right_value))
    })
}

fn main() {
    let left_input = String::from("alpha");
    let right_input = String::from("beta");
    let result = parallel_pair(|| left_input.len(), || right_input.to_ascii_uppercase());

    match result {
        Ok((length, upper)) => {
            assert_eq!(length, 5);
            assert_eq!(upper, "BETA");
        }
        Err(ParallelError::Start(error)) => panic!("test worker could not start: {error}"),
        Err(ParallelError::WorkerPanicked) => panic!("test worker panicked"),
    }
}
```

`Builder::spawn_scoped` reports operating-system thread creation failure as an `io::Result`.[^thread-builder] This executor's task-body contract requires normal return. It defensively classifies a worker panic; a panic in the calling-thread task follows ordinary unwinding and scope cleanup rather than becoming `WorkerPanicked`. The example does not claim that starting threads is a pure function.

#### 7.3 Independence and ordering

Tasks are **independent for parallel evaluation** when neither requires a result or state update from the other and their permitted execution order does not change the agreed observations. Reading the same immutable input can satisfy this requirement. Updating the same logical state, sharing a sequence-dependent generator, or performing ordered external writes generally requires a different contract.

A parallel map over ordered input MUST define output ordering. Preserving source order normally means associating each result with its input position, irrespective of completion order. Completion order MAY be exposed, but it must be an intentional part of the result type or API description.

Error selection MUST also be specified. The first error by source position, the first observed worker failure, and all errors in source order are different policies. A deterministic domain API SHOULD prefer a stable policy when callers do not need completion-order information.

#### 7.4 Laws for successful evaluation

For independent, pure, normally returning tasks and a successful execution mechanism, the basic result law is:

```text
run_parallel(pair(task_f, task_g)) ≃ (task_f(), task_g())
```

The equation compares the resulting pair, not elapsed time or thread creation. A value transformation over a completed result SHOULD preserve that result's position and failure interpretation. Introducing a parallel execution step MUST NOT silently change a task's domain inputs, duplicate it, or discard its result.

Parallel reduction has an additional requirement. Regrouping a reduction requires associativity of its combining operation over the actual domain. Reordering inputs additionally requires commutativity or another domain-specific justification. These are separate properties, developed in [Chapter 10](#chapter-10).

#### 7.5 Bounded execution and progress

Parallelism MUST have a bounded resource policy appropriate to the input size. A program SHOULD choose useful work units rather than treating every scalar operation as an independent operating-system thread. The executor above has a fixed maximum of one additional worker for its single pair; repeatedly invoking it is a separate resource decision.

Dependencies between tasks MUST admit progress under the configured execution resources. Blocking external work SHOULD have an execution policy distinct from CPU-bound value transformation. A functional decomposition makes dependencies easier to see, but the executor is still responsible for progress, shutdown, and resource limits.

A join handle is an execution capability. Its ownership and drop behavior MUST be understood separately from the logical result it eventually provides. In particular, the lifecycle of an ordinary spawned thread is not represented merely by the lifetime of a local handle.[^join-handle]

#### 7.6 Testing parallel interpretations

Tests SHOULD compare a parallel interpretation against a sequential reference computation for the same inputs. They MUST also verify the promised output order, worker limit, error policy, and completion behavior. Pure callbacks make the result comparison deterministic; deliberately injected failures test the boundary rather than the domain algorithm.

A performance claim requires measurement on representative inputs and the actual execution policy. The fact that two computations can run in parallel is not evidence that executing them that way is cheaper.

<a id="chapter-8"></a>
### 8. Property-Based Testing

#### 8.1 Properties, generators, and observations

A **property** is a predicate expressing a requirement over a class of inputs. A **generator** constructs test inputs under explicit size and distribution policies. A **test runner** evaluates the property on generated cases and records enough information to diagnose a failure. **Shrinking** searches simpler candidate inputs that preserve the failure. This separation follows the book's design of a property-testing library.[^b08]

Properties SHOULD express independent facts about the intended behavior rather than restating an implementation line by line. For sorting, useful properties include ordering, preservation of multiplicities, and idempotence. Ordering alone admits an implementation that returns an empty collection for every input.

Every property MUST state its domain and equality. For a state machine, the observed state is part of the result. For an iterator, values, termination, and consumption may matter. For an interpreter, a trace of attempted actions can be more useful than only its final return value.

#### 8.2 A deterministic generated test

The following example separates a repeatable input generator from a sorting property. Its word generator is a specified test-data mechanism, not a claim about the statistical suitability of its samples. All arithmetic in that generator is explicitly wrapping.

```rust
// Example 14: Generated inputs and independently specified properties.
use std::collections::BTreeMap;

fn next_word(state: u64) -> (u64, u64) {
    let next = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    (next, next)
}

fn generate_vec(mut state: u64, size: usize) -> (Vec<i16>, u64) {
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        let (word, next) = next_word(state);
        state = next;
        let bytes = word.to_le_bytes();
        values.push(i16::from_le_bytes([bytes[4], bytes[5]]));
    }
    (values, state)
}

fn histogram(values: &[i16]) -> BTreeMap<i16, usize> {
    let mut counts = BTreeMap::new();
    for &value in values {
        *counts.entry(value).or_insert(0) += 1;
    }
    counts
}

fn sorting_property(input: &[i16]) -> bool {
    let mut sorted = input.to_vec();
    sorted.sort_unstable();
    let mut sorted_twice = sorted.clone();
    sorted_twice.sort_unstable();

    sorted.windows(2).all(|pair| pair[0] <= pair[1])
        && histogram(input) == histogram(&sorted)
        && sorted == sorted_twice
}

fn main() {
    let mut state = 0x9a37_041f_8520_6bd1_u64;
    for case in 0..512 {
        let case_seed = state;
        let (input, next) = generate_vec(state, case % 65);
        state = next;
        assert!(
            sorting_property(&input),
            "seed={case_seed}, case={case}, input={input:?}",
        );
    }

    for input in [vec![], vec![0], vec![i16::MAX, i16::MIN, i16::MAX]] {
        assert!(sorting_property(&input));
    }
}
```

The explicit edge cases complement generated cases. The generation size is bounded, and the failure report identifies the actual input, case index, and seed. The histogram's count is bounded by the length of a valid `i16` slice. Sorting is performed on owned copies, leaving the observed input unchanged.[^slice]

#### 8.3 Generators must cover the contract

Generators SHOULD construct valid structured values directly when testing valid-input laws. Separate generators SHOULD exercise rejected inputs. Filtering arbitrary data until a complex invariant happens to hold can hide both a poor coverage distribution and an unbounded search.

A generator's size policy MUST constrain recursive depth, collection length, and other expensive dimensions. Size is not necessarily the number of bytes: a short description can request a large computation. Inputs near numeric limits, empty and singleton collections, repeated elements, distinct enum variants, and multibyte text SHOULD receive deliberate coverage where relevant.

Tests of higher-order operations SHOULD use several deterministic function families, including functions that preserve, discard, rearrange, and reject values in different ways. Testing a mapping law with the identity callback alone is not a test of composition.

#### 8.4 Shrinking and reproducibility

A shrinker MUST preserve the predicate's input preconditions or explicitly distinguish invalid candidates. It SHOULD simplify structure as well as scalar values: remove sequence segments, reduce tree branches, shorten text, and move numbers toward relevant boundaries. A shrink candidate is useful only after the property is reevaluated on it.

The runner MUST retain a still-failing candidate when a proposed simplification passes. Shrinking MUST have a termination policy and SHOULD avoid repeatedly revisiting the same candidate. A reported counterexample MUST reproduce the failure under the recorded configuration.

Shrinking produces a smaller counterexample relative to a search strategy and budget. A runner SHOULD NOT label it globally minimal unless that stronger statement has been established. Seed replay can be affected by changes to the generator, so a durable regression test SHOULD retain the concrete failing value as well.

#### 8.5 Law suites and model-based tests

An interface adopting an algebraic structure MUST test its defining laws on representative domains. A monoid needs left identity, right identity, and associativity. Mapping needs identity and composition. Monadic sequencing needs both identity laws and associativity. Accumulating validation additionally needs stable error ordering and preservation of all selected failures.

A state machine SHOULD be tested with generated event sequences against a simpler reference model. After each accepted event, verify the state invariant; after rejection, verify the documented failure-state policy. Testing only the final counter value can miss an invalid intermediate transition.

Metamorphic properties relate multiple legitimate executions. Examples include normalizing twice, splitting and recombining a monoidal reduction, and changing an executor without changing successful ordered results. The input transformation used by a metamorphic property MUST preserve the assumptions of that property.

#### 8.6 Failure classification and evidence

A runner MUST distinguish a falsified property from exhausted generation, a timeout, and an execution-environment failure. An unexpected panic is evidence of failure to satisfy the tested contract, not a successful rejected input, unless the property explicitly concerns that panic behavior.

Passing generated tests is evidence from the tested cases. Exhaustive evaluation of a finite declared domain can establish the property for that domain. An algebraic proof can establish a general law under stated assumptions. These forms of evidence SHOULD be reported accurately and combined where useful; none should be presented as another.

<a id="chapter-9"></a>
### 9. Parser Combinators

#### 9.1 A parser is a stateful value transformation

A **parser** transforms an input position into either a semantic result and a following position or a structured parse failure. A **parser combinator** constructs a parser from other parsers. A pure parser reads only its supplied input and immutable grammar configuration. The book develops parser APIs from their algebra, then specifies error reporting and backtracking explicitly.[^b09]

A parser interface MUST define its input unit, cursor invariant, success result, consumption behavior, and failure representation. For UTF-8 text, a byte offset is useful for slicing and source spans, but a valid text cursor must remain on a character boundary. A byte offset, a Unicode scalar count, and a displayed column are different measurements.[^str]

Parsing and domain validation SHOULD remain identifiable operations. A syntactically valid number can still violate a domain range. A parser may combine these stages, but its error types and documentation must preserve the distinctions required by callers.

#### 9.2 Primitive operations and derived structure

Useful primitives recognize a literal, recognize an input item, return a value without consuming input, or fail with a diagnostic. Mapping changes a parsed value without changing the input it consumed. Sequencing runs the following parser at the first parser's resulting position. Dependent sequencing may choose that following parser from the first parsed value.

**Choice** requires a policy. This specification uses ordered choice: try the left parser first and try the right parser at the original position only after an uncommitted failure. A failure is **committed** when the grammar has accepted enough input to make ordinary fallback inappropriate. An explicit `attempt` permits fallback after a failing branch while retaining the failure's diagnostic position.

Commitment is not a global preference for one error message. It is information that controls which branches execute. A parser MUST preserve that information when composing failures.

#### 9.3 A compact parser algebra

The following implementation produces owned or input-independent semantic values. The cursor borrows the input, and literals are static strings. Literal matching is atomic: a literal mismatch reports its starting offset and is uncommitted. Sequencing commits a later failure after its first parser consumed input. Ordered choice retains the farther diagnostic, preferring the left diagnostic on a tie, while preserving the right branch's commitment when that branch was attempted.

```rust
// Example 15: Pure parsing with consumption, choice, and bounded repetition.
#[derive(Clone, Copy)]
struct Input<'a> {
    text: &'a str,
    offset: usize,
}

impl<'a> Input<'a> {
    fn remaining(self) -> &'a str {
        &self.text[self.offset..]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseError {
    offset: usize,
    expected: &'static str,
    committed: bool,
}

type ParseResult<'a, A> = Result<(A, Input<'a>), ParseError>;
struct Parser<A>(Box<dyn for<'a> Fn(Input<'a>) -> ParseResult<'a, A>>);

impl<A: 'static> Parser<A> {
    fn new<F>(parse: F) -> Self
    where
        F: for<'a> Fn(Input<'a>) -> ParseResult<'a, A> + 'static,
    {
        Self(Box::new(parse))
    }

    fn map<B: 'static, F>(self, transform: F) -> Parser<B>
    where
        F: Fn(A) -> B + 'static,
    {
        Parser::new(move |input| {
            let (value, next) = (self.0)(input)?;
            Ok((transform(value), next))
        })
    }

    fn and_then<B: 'static, F>(self, next_parser: F) -> Parser<B>
    where
        F: Fn(A) -> Parser<B> + 'static,
    {
        Parser::new(move |input| {
            let start = input.offset;
            let (value, next) = (self.0)(input)?;
            (next_parser(value).0)(next).map_err(|mut error| {
                error.committed |= next.offset > start;
                error
            })
        })
    }

    fn or(self, alternative: Self) -> Self {
        Self::new(move |input| {
            let left_error = match (self.0)(input) {
                Ok(success) => return Ok(success),
                Err(error) if error.committed => return Err(error),
                Err(error) => error,
            };
            match (alternative.0)(input) {
                Ok(success) => Ok(success),
                Err(right_error) => {
                    let committed = right_error.committed;
                    let mut selected = if left_error.offset >= right_error.offset {
                        left_error
                    } else {
                        right_error
                    };
                    selected.committed = committed;
                    Err(selected)
                }
            }
        })
    }

    fn attempt(self) -> Self {
        Self::new(move |input| {
            (self.0)(input).map_err(|mut error| {
                error.committed = false;
                error
            })
        })
    }

    fn repeat_at_most(self, limit: usize) -> Parser<Vec<A>> {
        Parser::new(move |start| {
            let mut input = start;
            let mut values = Vec::new();
            for _ in 0..limit {
                match (self.0)(input) {
                    Ok((value, next)) => {
                        if next.offset <= input.offset {
                            return Err(ParseError {
                                offset: input.offset,
                                expected: "input progress",
                                committed: true,
                            });
                        }
                        values.push(value);
                        input = next;
                    }
                    Err(error) if !error.committed => return Ok((values, input)),
                    Err(error) => return Err(error),
                }
            }
            Ok((values, input))
        })
    }

    fn parse_all(&self, text: &str) -> Result<A, ParseError> {
        let (value, next) = (self.0)(Input { text, offset: 0 })?;
        if next.offset == text.len() {
            Ok(value)
        } else {
            Err(ParseError {
                offset: next.offset,
                expected: "end of input",
                committed: next.offset > 0,
            })
        }
    }
}

fn literal(token: &'static str) -> Parser<()> {
    Parser::new(move |input| {
        if input.remaining().starts_with(token) {
            Ok((
                (),
                Input {
                    text: input.text,
                    offset: input.offset + token.len(),
                },
            ))
        } else {
            Err(ParseError {
                offset: input.offset,
                expected: token,
                committed: false,
            })
        }
    })
}

fn branch(suffix: &'static str) -> Parser<&'static str> {
    literal("a").and_then(move |_| literal(suffix).map(move |_| suffix))
}

fn main() {
    assert_eq!(literal("record").map(|_| 7).parse_all("record"), Ok(7));
    assert!(literal("record").parse_all("record!").is_err());
    assert!(branch("b").or(branch("c")).parse_all("ac").is_err());
    assert_eq!(
        branch("b").attempt().or(branch("c")).parse_all("ac"),
        Ok("c"),
    );

    let repeated = literal("λ").repeat_at_most(3);
    assert_eq!(repeated.parse_all("λλλ").unwrap().len(), 3);
    assert!(repeated.parse_all("λλλλ").is_err());
    assert!(literal("").repeat_at_most(1).parse_all("").is_err());
}
```

The higher-ranked callable bound expresses that the same parser can receive inputs with different borrowing lifetimes. It does not require the input text to have a static lifetime. The static bound applies to stored grammar behavior and semantic type parameters in this representation.[^higher-ranked]

#### 9.4 Parser laws

A mapping operation MUST preserve success consumption and failure information while changing only a successful semantic value. Under pure result transformations, identity and composition therefore have the following meaning:

```text
run(map(p, identity), input)       ≃ run(p, input)
run(map(map(p, f), g), input)      ≃ run(map(p, g ∘ f), input)
```

Sequencing MUST begin the second parser at the position returned by the first. A first-stage failure MUST prevent dependent parsing. Ordered choice MUST preserve a successful left result without evaluating its alternative. An uncommitted left failure MUST restart the alternative at the original position, not at the failure's diagnostic offset.

`attempt` MUST preserve successful results and positions. On failure, it changes fallback permission without pretending that the diagnostic occurred at a different position. Ordered choice is intentionally left-biased; exchanging its branches can change both accepted results and reported errors.

A complete-input entry point MUST reject trailing input. A prefix parser MAY return a remaining position instead, but callers must deliberately choose that contract.

#### 9.5 Repetition, progress, and bounds

A repetition combinator MUST specify its minimum and maximum item counts. Unbounded repetition requires a proof that each successful iteration consumes input or otherwise makes a well-founded state transition. The example enforces positive consumption for every repeated success and permits at most the supplied number of successful items.

Reaching the example's bound returns the accumulated prefix. `parse_all` then rejects any remaining input. This is a bounded-prefix contract, not a separate diagnostic for exceeding an item limit. A production grammar that needs that diagnostic SHOULD make it explicit.

Input size, nesting depth, repetition count, and backtracking work MUST be bounded where the parser accepts untrusted input. A bound on returned items alone does not bound the amount of work inside each attempted parse. The budget must follow actual grammar operations.

#### 9.6 Diagnostics and semantic validation

A useful parse failure SHOULD identify the position, expected construct, and relevant grammar context. Diagnostic combination MUST define how ties are handled and whether alternatives are retained. Formatting a user-facing message SHOULD be separate from choosing which failure the parser reports.

A failed literal, an unfinished construct, a numeric overflow, and a domain constraint violation are different facts. A parser SHOULD not erase these distinctions merely to expose one generic error message. At the same time, retaining the entire input in every error can extend the lifetime of large or sensitive buffers; error representations SHOULD retain only the context their consumers require.

The algebra is complete only when its failure and consumption behavior is compositional. A grammar that parses valid examples but handles malformed input unpredictably has not established that contract.

<a id="part-iii"></a>
## Part III. Common Structures in Functional Design

An algebraic abstraction identifies operations and laws shared by otherwise different domains. Its value is the reasoning and reuse those laws permit. This part retains the book's progression from associative combination to mapping, dependent sequencing, independent combination, and traversal.[^b10][^b11][^b12]

Names of mathematical structures MUST be accompanied by their defining laws. The laws below use schematic notation for contextual values; the Rust implementations use concrete types and their ordinary methods. An application SHOULD introduce only the abstractions that clarify its actual contracts.

<a id="chapter-10"></a>
### 10. Monoids and Lawful Reduction

#### 10.1 The algebra

A **semigroup** consists of a carrier of values and an associative binary operation closed over that carrier. A **monoid** adds a two-sided identity. Write the operation as `⊕` and the identity as `e`:

```text
Closure:        a ⊕ b belongs to the same carrier.
Associativity:  (a ⊕ b) ⊕ c ≃ a ⊕ (b ⊕ c)
Left identity:  e ⊕ a ≃ a
Right identity: a ⊕ e ≃ a
```

The operation MUST be defined throughout the stated domain, and the equality used by the laws MUST be preserved by combination. A monoid is the carrier together with the operation and identity, not a property of a type name in isolation. The same representation can support different monoids with different meanings.[^b10]

String concatenation has an empty-string identity. Ordered sequence concatenation has an empty-sequence identity. Boolean conjunction has `true` as identity, while disjunction has `false`. These examples share laws, not application meaning. Concatenation is generally not commutative: changing the order of operands changes the result.

#### 10.2 An ownership-oriented Rust interface

A consuming combination operation can reuse storage owned by one of its arguments. Newtypes distinguish different interpretations of a representation. The following trait is a contract: its implementation syntax supplies operations, while the implementer remains responsible for the laws.

```rust
// Example 16: Monoids, product composition, and an explicitly modular sum.
use std::num::Wrapping;

trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModularSum(Wrapping<u64>);

impl Monoid for ModularSum {
    fn empty() -> Self {
        Self(Wrapping(0))
    }

    fn combine(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Text(String);

impl Monoid for Text {
    fn empty() -> Self {
        Self(String::new())
    }

    fn combine(mut self, other: Self) -> Self {
        self.0.push_str(&other.0);
        self
    }
}

impl<A: Monoid, B: Monoid> Monoid for (A, B) {
    fn empty() -> Self {
        (A::empty(), B::empty())
    }

    fn combine(self, other: Self) -> Self {
        (self.0.combine(other.0), self.1.combine(other.1))
    }
}

fn fold_map<I, M, F>(input: I, transform: F) -> M
where
    I: IntoIterator,
    M: Monoid,
    F: FnMut(I::Item) -> M,
{
    input
        .into_iter()
        .map(transform)
        .fold(M::empty(), M::combine)
}

fn main() {
    let summary: (Text, ModularSum) = fold_map([("a", u64::MAX), ("b", 1)], |(text, value)| {
        (Text(text.to_owned()), ModularSum(Wrapping(value)))
    });
    assert_eq!(summary.0, Text(String::from("ab")));
    assert_eq!(summary.1, ModularSum(Wrapping(0)));

    let samples = [0, 1, u64::MAX].map(|n| ModularSum(Wrapping(n)));
    for a in samples {
        assert_eq!(ModularSum::empty().combine(a), a);
        assert_eq!(a.combine(ModularSum::empty()), a);
        for b in samples {
            for c in samples {
                assert_eq!(a.combine(b).combine(c), a.combine(b.combine(c)));
            }
        }
    }
}
```

`Wrapping<u64>` gives modular arithmetic explicitly.[^wrapping] The result above is a modular checksum-like sum, not an exact unbounded count. Its associativity follows from addition modulo `2^64`. Text combination preserves ordered contents while permitting local reuse of owned storage.

The product instance combines each component independently. Its laws follow componentwise: each component has the required identity, and each component's combination is associative. No commutativity assumption is needed.

#### 10.3 Folds, partitions, and homomorphisms

A monoidal fold of a finite ordered sequence may regroup combinations without changing the result. Left and right association agree when the identity and operation satisfy the laws and element order is unchanged. A general accumulator fold does not require these laws unless such regrouping is performed.

For a pure mapping function and a lawful monoid, the partition law is:

```text
fold_map(xs ++ ys, f) ≃ fold_map(xs, f) ⊕ fold_map(ys, f)
fold_map([], f)       ≃ e
```

This law permits independent summaries of chunks followed by ordered combination. Chunk boundaries SHOULD be treated as an implementation parameter and tested with empty, singleton, uneven, and multiple-chunk partitions.

A **monoid homomorphism** preserves the identity and combination between two monoids:

```text
h(e_M)      ≃ e_N
h(a ⊕_M b)  ≃ h(a) ⊕_N h(b)
```

Such a mapping permits a summary to move across a combination boundary without changing its meaning. An optimization claiming this property MUST establish it for the actual summary and operation. A summary that discards information needed to combine adjacent chunks does not have that property merely because it is small.

#### 10.4 Numeric semantics are algebraic semantics

Machine arithmetic MUST be included in the law, not treated as an implementation afterthought. Ordinary bounded integer operators have overflow behavior that depends on the operation and configured checks. Explicitly checked, wrapping, and saturating operations each define their own semantics.[^operators]

A fallible checked operation is not automatically a monoid over its successful numeric type. Even lifting checked signed addition into a failure-propagating result does not automatically establish associativity. For example, one grouping of a maximum signed value, `1`, and `-1` encounters overflow, while another grouping first combines `1` with `-1` and avoids it. The error is an observable result.

Floating-point addition is not associative under exact result equality. Different groupings can round differently; NaN also requires special care when selecting an equality relation. A floating-point reduction MUST specify the reproducibility or approximation contract it actually provides. A tolerance-based test is not, by itself, a proof of an associative algebra.[^float]

An exact domain total SHOULD use a representation and overflow policy appropriate to that domain. A modular checksum SHOULD say that it is modular. These are different contracts even when both use a machine integer underneath.

#### 10.5 Order and parallel reduction

Associativity permits changing parentheses. Commutativity permits exchanging operands. An ordered concatenation can therefore be reduced in a balanced tree while preserving chunk order; arbitrary completion-order concatenation would change its result.

A parallel reducer MUST document both the grouping freedom and the input-order freedom it uses. A result described as deterministic MUST remain the same under the supported scheduling choices. Failure precedence and resource outcomes belong to the executor's contract in addition to the algebra of successful values.

Empty-input behavior MUST be explicit. A monoidal fold returns its identity. A reduction with no identity SHOULD return an optional result or require nonempty input rather than inventing a successful sentinel.

#### 10.6 Composing useful summaries

Several independent summaries can be combined as a product and computed in one traversal. Examples include minimum and maximum observations, accumulated flags and diagnostics, or an ordered rendering and a modular checksum. Each component MUST retain its own meaning and laws.

A summary SHOULD be no more general than its consumers require, but it must retain enough information to compose correctly. When a result depends on boundaries between adjacent chunks, those boundary facts belong in the summary. Deriving the combination rule from the intended whole-input meaning is more reliable than choosing a convenient field layout and hoping it combines.

<a id="chapter-11"></a>
### 11. Functors and Monadic Composition

#### 11.1 Contexts and mapping

A **contextual value** carries a value together with a computational interpretation: possible absence, possible rejection, several ordered alternatives, an explicit state transition, or another specified context. A **functor**, in this reference, is a family of contextual value types with a mapping operation that preserves identity and composition.[^b11]

For a context written schematically as `C<A>`, mapping has the shape:

```text
map: (C<A>, A -> B) -> C<B>

Identity:    map(value, identity) ≃ value
Composition: map(map(value, f), g) ≃ map(value, g ∘ f)
```

For `Option`, mapping preserves absence or presence. For `Result<T, E>` with a fixed error type, mapping preserves errors and transforms successes. For an ordered sequence, mapping preserves length and positions. For a state action, mapping preserves the resulting state and transforms only its result component.

A mapping operation MUST preserve the context's relevant structure. It MUST NOT insert an unrelated rejection, silently remove an element, or advance a different state solely because the successful value's type changes. Its callbacks must satisfy the semantic assumptions used by the laws.

#### 11.2 Injection and dependent sequencing

An **injection**, written `pure`, embeds an ordinary value into a context without adding that context's additional behavior. Examples are `Some(value)`, `Ok(value)`, a singleton vector, and an unchanged-state action returning `value`.

A **bind** operation, written schematically as `bind` and commonly expressed in Rust by `and_then`, sequences a function that returns another value in the same context:

```text
pure: A -> C<A>
bind: (C<A>, A -> C<B>) -> C<B>
```

A **monad** is such a context with injection and bind satisfying the identity and associativity laws. The practical interpretation is dependent composition: the next computation may use a successful result of the preceding computation.[^b11]

For `Option`, bind propagates absence. For `Result`, it propagates the first error in the dependent chain. For vectors, a corresponding ordered operation concatenates the alternatives returned for successive elements; `into_iter().flat_map(...)` expresses that traversal without requiring an intermediate vector for each outer stage. For state actions, bind threads the new state into the chosen continuation.[^option][^result][^iter-module]

The callback invocation contract depends on the concrete context. An optional or fallible single value invokes a continuation at most once. A sequence may invoke it for many elements. Ownership and callable bounds SHOULD follow that difference.

#### 11.3 Monad laws

The laws are:

```text
Left identity:  bind(pure(a), f)       ≃ f(a)
Right identity: bind(m, pure)          ≃ m
Associativity:  bind(bind(m, f), g)    ≃ bind(m, a -> bind(f(a), g))
```

The associativity law changes the grouping of dependent composition, not its semantic order. It does not exchange `f` and `g`. For a state context, equality includes the final state. For a result context, equality includes the error chosen. For ordered alternatives, equality includes order and multiplicity.

The laws are evaluated on legitimate inputs and pure, normally returning continuations in the value model. They do not authorize duplication of ownership or disregard observable capture destruction. A test of a one-shot action MUST build a corresponding action for each side rather than attempt to reuse consumed execution state.

```rust
// Example 17: Mapping and bind laws for concrete optional values.
fn first(value: u8) -> Option<u8> {
    if value % 2 == 0 {
        value.checked_add(2)
    } else {
        None
    }
}

fn second(value: u8) -> Option<u8> {
    value.checked_mul(3)
}

fn main() {
    let values = std::iter::once(None).chain((u8::MIN..=u8::MAX).map(Some));
    for value in values {
        assert_eq!(value.map(std::convert::identity), value);
        assert_eq!(
            value.map(u16::from).map(|n| n * 2),
            value.map(|n| u16::from(n) * 2),
        );
        assert_eq!(value.and_then(Some), value);
        assert_eq!(
            value.and_then(first).and_then(second),
            value.and_then(|n| first(n).and_then(second)),
        );
    }
    for value in u8::MIN..=u8::MAX {
        assert_eq!(Some(value).and_then(first), first(value));
    }
}
```

This exhaustively covers the `Option<u8>` input values for the selected callbacks. It does not quantify over every possible callback implementation. The multiplication in the mapping test is safe because converting a `u8` and doubling it produces at most `510`, within `u16`.

#### 11.4 Mapping, flattening, and bind agree

Injection and bind determine a mapping operation. Bind and the identity function determine flattening of one contextual layer:

```text
map(m, f)       ≃ bind(m, a -> pure(f(a)))
flatten(nested) ≃ bind(nested, identity)
bind(m, f)      ≃ flatten(map(m, f))
```

An interface exposing these operations MUST keep their meanings coherent. For a result type, flattening combines nested success-or-error layers with the same error interpretation; it does not accumulate independent errors. For optional values, flattening preserves absence in either layer.

These equations concern the declared value interpretation. A materialized vector of intermediate results and a lazy iterator may differ operationally in allocation and retention even when a completed traversal has the same values. Such differences must remain within the interface's permitted observations.

#### 11.5 Kleisli composition

A function `A -> C<B>` is a contextual arrow. **Kleisli composition** composes two such functions by binding the first result into the second. It is a useful way to define a reusable checked pipeline without changing the ordinary functions at its stages.

```rust
// Example 18: Composition of functions that return Result.
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueError {
    Parse,
    Zero,
}

fn chain_result<A, B, C, E, F, G>(first: F, second: G) -> impl FnOnce(A) -> Result<C, E>
where
    F: FnOnce(A) -> Result<B, E>,
    G: FnOnce(B) -> Result<C, E>,
{
    move |input| first(input).and_then(second)
}

fn parse_value(text: &str) -> Result<u32, ValueError> {
    text.parse().map_err(|_| ValueError::Parse)
}

fn require_nonzero(value: u32) -> Result<NonZeroU32, ValueError> {
    NonZeroU32::new(value).ok_or(ValueError::Zero)
}

fn main() {
    let checked = chain_result(parse_value, require_nonzero);
    assert_eq!(checked("23").map(NonZeroU32::get), Ok(23));
    assert_eq!(
        chain_result(parse_value, require_nonzero)("0"),
        Err(ValueError::Zero)
    );
    assert_eq!(
        chain_result(parse_value, require_nonzero)("x"),
        Err(ValueError::Parse)
    );
}
```

Under the monad laws, this composition is associative and `pure` acts as its identity arrow. Error conversions MUST be explicit when stages use different error types. A shared domain error enum or a deliberate `map_err` can establish the common channel.

The example's parser delegates to the standard `u32` syntax and intentionally classifies all numeric parse failures as `ValueError::Parse`. It separates that classification from the nonzero domain check.

#### 11.6 Environments and accumulated descriptions

An immutable environment can be passed as a shared argument to several pure functions. Schematically, `Config -> A` describes a computation that depends on configuration. Sequencing two such computations supplies the same logical environment to both; a following computation can also use the first result. In Rust, ordinary functions taking `&Config` or closures capturing an immutable configuration value express this pattern directly.

A computation that returns `(A, Vec<Diagnostic>)` separates a value from descriptions produced while computing it. Sequencing combines the first diagnostic sequence with the second in order. The empty sequence is the injection's diagnostic component, and sequence concatenation supplies the associative combination. Formatting or publishing these diagnostics is a later interpretation.

The environment MUST contain stable logical inputs for a pure claim. A diagnostic collection MUST be treated as returned data rather than a hidden logging sink. Neither pattern requires a special wrapper when a parameter or tuple already communicates the contract.

#### 11.7 Choosing the operation that exposes the dependency

Use a plain function for an ordinary transformation, `map` for a transformation inside a context, and `and_then` or `?` when the next computation itself returns an optional or fallible value. Use an explicit state transition when the following computation needs a new state, and an independent combination when several validations can be performed without each other's successful values.

A function SHOULD not accept an optional or fallible input merely because one particular caller has such a value. Keep the ordinary domain function when its inputs are ordinary values, then lift it with the appropriate contextual operation at the call site. This preserves separation between the domain calculation and the caller's sequencing policy.

<a id="chapter-12"></a>
### 12. Applicative Validation and Traversal

#### 12.1 Independent contextual combination

An **applicative** context supports injection, mapping, and combination of independent contextual values with an ordinary function. The central binary operation has the schematic shape:

```text
map2: (C<A>, C<B>, (A, B) -> D) -> C<D>
```

The second contextual value is supplied independently of the first value's successful contents. The operation may therefore specify an error-accumulating interpretation. This distinction between dependent sequencing and independent combination underlies the book's validation example.[^b12]

Independence describes information dependency, not execution timing. An implementation may evaluate independent validations sequentially. An executor may run independent jobs concurrently. Both must preserve the combination policy specified by the context.

Rust evaluates ordinary argument expressions before a function call. A `map2` receiving two already-computed results is not controlling whether those expressions ran. A combinator that needs to control their evaluation must receive suitable deferred computations.[^expressions]

#### 12.2 Nonempty error accumulation

A rejected validation SHOULD carry at least one diagnostic. Independent failures MUST be combined in a defined order. A success constructor represents the absence of validation errors; an invalid value with an empty error list should not be another representation of success.

```rust
// Example 19: Applicative validation and an accumulating traversal.
use std::num::NonZeroU16;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Errors<E> {
    first: E,
    rest: Vec<E>,
}

impl<E> Errors<E> {
    fn one(error: E) -> Self {
        Self {
            first: error,
            rest: Vec::new(),
        }
    }

    fn append(mut self, other: Self) -> Self {
        self.rest.push(other.first);
        self.rest.extend(other.rest);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Validation<T, E> {
    Valid(T),
    Invalid(Errors<E>),
}

impl<T, E> Validation<T, E> {
    fn map<U, F>(self, transform: F) -> Validation<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Valid(value) => Validation::Valid(transform(value)),
            Self::Invalid(errors) => Validation::Invalid(errors),
        }
    }

    fn map2<U, V, F>(self, other: Validation<U, E>, combine: F) -> Validation<V, E>
    where
        F: FnOnce(T, U) -> V,
    {
        match (self, other) {
            (Validation::Valid(a), Validation::Valid(b)) => Validation::Valid(combine(a, b)),
            (Validation::Invalid(a), Validation::Invalid(b)) => Validation::Invalid(a.append(b)),
            (Validation::Invalid(errors), _) | (_, Validation::Invalid(errors)) => {
                Validation::Invalid(errors)
            }
        }
    }
}

fn traverse_validation<I, U, E, F>(input: I, mut validate: F) -> Validation<Vec<U>, E>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> Validation<U, E>,
{
    input
        .into_iter()
        .fold(Validation::Valid(Vec::new()), |acc, item| {
            acc.map2(validate(item), |mut values, value| {
                values.push(value);
                values
            })
        })
}

fn validate_name(text: &str) -> Validation<String, &'static str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        Validation::Invalid(Errors::one("name must not be blank"))
    } else {
        Validation::Valid(trimmed.to_owned())
    }
}

fn validate_limit(text: &str) -> Validation<NonZeroU16, &'static str> {
    match text.parse::<NonZeroU16>() {
        Ok(value) => Validation::Valid(value),
        Err(_) => Validation::Invalid(Errors::one("limit must be a nonzero u16")),
    }
}

fn main() {
    let valid =
        validate_name(" Batch ").map2(validate_limit("8"), |name, limit| (name, limit.get()));
    assert_eq!(valid, Validation::Valid((String::from("Batch"), 8)));
    assert_eq!(
        validate_name("Batch").map(|name| name.len()),
        Validation::Valid(5)
    );

    let invalid = validate_name(" ").map2(validate_limit("0"), |name, limit| (name, limit.get()));
    assert_eq!(
        invalid,
        Validation::Invalid(Errors {
            first: "name must not be blank",
            rest: vec!["limit must be a nonzero u16"],
        })
    );

    let mut visited = 0;
    let batch = traverse_validation(["0", "7", "invalid"], |text| {
        visited += 1;
        validate_limit(text)
    });
    assert_eq!(visited, 3);
    match batch {
        Validation::Invalid(errors) => assert_eq!(errors.rest.len(), 1),
        Validation::Valid(_) => panic!("two invalid fields should have been reported"),
    }
}
```

The name validator trims whitespace. The limit validator uses the standard parser for nonzero `u16` values and deliberately emits one field-level diagnostic for any rejected limit. Its error policy differs from the finer-grained constructor in Chapter 4; that distinction is explicit.

The accumulating traversal invokes the validator for each input in order. Its fold does not short-circuit when the accumulator becomes invalid. The combining closure constructs the successful output vector only while the combined result remains valid. The visitation counter is test instrumentation for that evaluation contract.

#### 12.3 Applicative laws

The mapping operation MUST satisfy the functor laws. Injection and independent combination MUST also agree with it. Using `product(x, y) = map2(x, y, pair)`, the principal laws are:

```text
Left identity:
map2(pure(()), x, (_, a) -> a) ≃ x

Right identity:
map2(x, pure(()), (a, _) -> a) ≃ x

Combination of injected values:
map2(pure(a), pure(b), f) ≃ pure(f(a, b))

Associativity, with tuple reassociation:
product(product(x, y), z)
  ≃ map(product(x, product(y, z)), (a, (b, c)) -> ((a, b), c))

Naturality of product:
product(map(x, f), map(y, g))
  ≃ map(product(x, y), (a, b) -> (f(a), g(b)))

Coherence:
map2(x, y, f) ≃ map(product(x, y), (a, b) -> f(a, b))
```

These laws preserve the order `x`, then `y`, then `z` in the context's interpretation; they do not require those values to be exchangeable. For accumulating validation, the error collection's associative concatenation makes grouping irrelevant while retaining diagnostic order. Nonempty errors need an associative combination, not an empty invalid value.

An implementation MUST NOT truncate diagnostics, deduplicate them, or sort them unless the selected diagnostic algebra specifies that behavior and satisfies the adopted laws under its corresponding equality.

#### 12.4 Traversal and sequencing

A **traversal** applies a contextual function to each position of a finite structure and combines the results while preserving the structure's successful shape. For an ordered vector and fallible validation:

```text
traverse_result: (Vec<A>, A -> Result<B, E>) -> Result<Vec<B>, E>
sequence_result: Vec<Result<A, E>> -> Result<Vec<A>, E>
```

Standard iterator mapping followed by `collect::<Result<Vec<_>, _>>()` supplies a fail-fast ordered traversal. Collecting optional values supplies the corresponding optional traversal. The accumulating traversal above supplies a different, explicitly all-errors interpretation.[^result-collection][^option-collection]

The traversal MUST preserve input positions in a successful output. It MUST specify whether it stops at failure or evaluates all selected positions, which error ordering it exposes, and whether a rejected result retains any partial successful output. For a tree or another shaped structure, success must preserve the corresponding shape rather than merely its element count.

A finite eager traversal MUST NOT be described as a bounded-memory streaming operation when it retains the entire successful result. Input iteration may be incremental while the result remains a complete collection.

#### 12.5 Traversal laws and valid fusion

The identity and mapping relationships are:

```text
traverse(xs, pure)         ≃ pure(xs)
traverse(xs, pure ∘ f)     ≃ pure(map(xs, f))
sequence(xs)              ≃ traverse(xs, identity)
```

Naturality relates two contextual interpretations. If a transformation `φ` preserves injection and independent combination, then:

```text
φ(traverse_C(xs, f)) ≃ traverse_D(xs, φ ∘ f)
```

For example, mapping a pure error conversion over a fail-fast result traversal agrees with converting the errors of each element operation under the same first-error policy. The preservation assumptions are essential; an arbitrary conversion between representations does not establish this law.

Composition concerns two nested contexts, not an accidental collapse of two failure policies. For `f: A -> F<B>` and `g: B -> G<D>`, and the usual nested applicative interpretation of `F<G<_>>`:

```text
traverse_(F∘G)(xs, a -> map_F(f(a), g))
  ≃ map_F(traverse_F(xs, f), bs -> traverse_G(bs, g))
```

For nested results, the two error layers remain distinct in this equation. By contrast, replacing “first validate every input with `f`, then validate the resulting values with `g`” by a single fail-fast traversal of `f(a).and_then(g)` can change which error wins. A `g` error in an early element may then precede an `f` error in a later element. Such a rewrite MUST preserve the application's declared phase and error-order policy.

#### 12.6 Stateful traversal

A traversal may produce transformed values while explicitly accumulating state. This is often called `map_accum`. It combines a mapping operation with the state-threading discipline of Chapter 6.

```rust
// Example 20: Preserve each mapped result while threading explicit state.
fn map_accum<S, I, B, F>(initial: S, input: I, mut step: F) -> (Vec<B>, S)
where
    I: IntoIterator,
    F: FnMut(S, I::Item) -> (B, S),
{
    let mut state = initial;
    let mut output = Vec::new();
    for item in input {
        let (value, next_state) = step(state, item);
        output.push(value);
        state = next_state;
    }
    (output, state)
}

fn main() {
    let (prefixes, final_text) = map_accum(
        String::new(),
        ['r', 'u', 's', 't'],
        |mut text, character| {
            text.push(character);
            (text.clone(), text)
        },
    );
    assert_eq!(prefixes, vec!["r", "ru", "rus", "rust"]);
    assert_eq!(final_text, "rust");
}
```

The cloned prefixes are intentional output snapshots. Their total storage is part of the output cost, not a hidden constant-memory property of state threading. A fallible variant MUST additionally specify the state returned on rejection and whether previously produced outputs are retained.

#### 12.7 Validation dependency boundaries

Independent field checks SHOULD be combined before constraints that require those fields' successful values. A cross-field constraint should run once its required validated inputs exist. Its own diagnostics may then be accumulated with other constraints at that same dependency level.

A pipeline SHOULD make these levels visible: lexical parsing, field validation, cross-field validation, and execution planning are often different phases. Combining them into one long chain can accidentally select fail-fast behavior where complete diagnostics were intended, or execute work before its prerequisites were established.

The appropriate abstraction follows the dependency: mapping transforms one value, bind uses a prior value to choose the next computation, applicative combination joins independent contextual values, and traversal repeats a chosen interpretation across a structure.

<a id="part-iv"></a>
## Part IV. Effects and I/O

<a id="chapter-13"></a>
### 13. External Effects and Interpreters

#### 13.1 Describing an interaction and performing it

An **external effect** is an interaction whose meaning includes something beyond returning a domain value: reading a device, writing a record, obtaining an external identifier, or changing a shared service. An **effect description** is data specifying a requested interaction. An **interpreter** gives that description an execution meaning. An interpreter may perform I/O, produce an in-memory trace, or evaluate against an explicit model state.

The book develops this distinction by factoring decisions and descriptions out of the procedures that execute them. The governing principle here is the same: make domain decisions without obtaining execution capabilities, and give the interpreter an explicit contract for acting on those decisions.[^b13]

A description MUST contain enough information to identify its intended operation without consulting hidden mutable configuration during planning. This does not require predicting the external operation's result. A read request describes what to read; its result becomes an explicit input to the next decision. A write request describes what to write; its completion or failure is an interpreter outcome.

Descriptions SHOULD use domain terminology. A proposed document revision, an ordered set of report records, or a requested lookup is usually more useful than a collection of unnamed callbacks. Ordinary functions remain appropriate when there is no need to inspect, retain, compare, or interpret the description separately.

#### 13.2 A report plan with two interpretations

The following plan has two instructions: emit a line and emit an empty line. Planning rejects empty labels and labels containing a carriage return or line feed; it otherwise preserves the supplied text. The first interpretation produces bytes in memory. The second writes the same bytes to a supplied writer and explicitly requests a flush.

```rust
// Example 21: Plan once; render as a value or execute against a writer.
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReportCommand {
    Line(String),
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InvalidLabel {
    index: usize,
}

fn plan_labels(labels: &[&str]) -> Result<Vec<ReportCommand>, InvalidLabel> {
    let mut plan = vec![
        ReportCommand::Line("Labels".to_owned()),
        ReportCommand::Blank,
    ];
    for (index, label) in labels.iter().enumerate() {
        if label.is_empty() || label.bytes().any(|byte| matches!(byte, b'\r' | b'\n')) {
            return Err(InvalidLabel { index });
        }
        plan.push(ReportCommand::Line((*label).to_owned()));
    }
    Ok(plan)
}

fn line_text(command: &ReportCommand) -> &str {
    match command {
        ReportCommand::Line(text) => text,
        ReportCommand::Blank => "",
    }
}

fn render(plan: &[ReportCommand]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for command in plan {
        bytes.extend_from_slice(line_text(command).as_bytes());
        bytes.push(b'\n');
    }
    bytes
}

fn execute<W: Write>(plan: &[ReportCommand], writer: &mut W) -> io::Result<()> {
    for command in plan {
        writer.write_all(line_text(command).as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()
}

fn main() {
    let plan = plan_labels(&["alpha", "beta"]).expect("valid fixture");
    let expected = b"Labels\n\nalpha\nbeta\n";
    assert_eq!(render(&plan).as_slice(), expected);

    let mut written = Vec::new();
    execute(&plan, &mut written).expect("in-memory writer");
    assert_eq!(written, render(&plan));
    assert_eq!(
        plan_labels(&["alpha", "bad\nlabel"]),
        Err(InvalidLabel { index: 1 })
    );
}
```

`render` is a value transformation. `execute` is an execution boundary: its behavior includes calls to the supplied writer. Testing `execute` with a local `Vec<u8>` is useful, but it does not establish that every possible writer has those same environmental properties. `write_all` handles successive writes until the supplied bytes have been written or an error occurs; it does not establish an all-or-nothing transaction. `flush` reports the writer's own flush outcome.[^write]

The value interpretation supports exact byte-level assertions without a production resource. An interpreter-equivalence property can state that a successful in-memory execution emits exactly the bytes returned by `render`. A property about successful bytes does not replace separate tests of partial writes and flush failures.

#### 13.3 Execution order and partial progress

An interpreter MUST specify instruction order, the point at which it stops on failure, and which previously completed operations remain visible. Example 21 executes commands in vector order, stops on the first write error, and flushes only after all commands have been written. An error may follow a partially written command. The caller retains ownership of the writer and responsibility for its subsequent handling.

Where an application requires an atomic publication, the execution protocol MUST establish that property explicitly. Returning a `Result`, building a complete plan, or checking all domain constraints before execution does not itself undo an external write. The pure planner's all-or-nothing result and the interpreter's partial external progress are separate contracts.

Retries MUST be justified by the operation's semantics. Repeating a request can produce an additional external action even when its description is unchanged. An interpreter that retries SHOULD distinguish an operation known not to have started from an operation whose completion is uncertain. A domain model may carry an explicit operation identity when the surrounding protocol uses that identity to recognize repeated requests.

#### 13.4 Capabilities and module boundaries

A module that makes decisions SHOULD depend on input values and domain types. The module that acquires or owns a resource SHOULD retain authority over its execution and release policy. Narrow writer, reader, or domain-service interfaces can make that authority explicit, but invoking such an interface remains an interaction.

Configuration SHOULD be validated at the boundary and passed inward as a stable value. Time SHOULD be sampled at a deliberate point and passed to the rule that needs it. Entropy SHOULD be obtained at the boundary and converted to explicit state or generated input before invoking a deterministic decision function. Tests can then use fixed configurations, times, and generated inputs without replacing the domain algorithm.

An interpreter implementation SHOULD be testable against controlled resources. Tests MUST establish the relevant interaction trace, including order and failure behavior, rather than merely checking the last returned value. A trace collector represents observed requests as data; it need not pretend that a request has succeeded in the real system.

#### 13.5 Asynchronous dependent composition

A Rust `async fn` returns a future representing its body. Calling it evaluates its arguments, but execution of that body proceeds when the future is polled. A `Future` represents a computation that may not yet have produced its output; polling advances it and may itself perform work or interactions. These language and trait contracts are distinct from a purity claim.[^async-functions][^future]

Dependent asynchronous composition has a direct implementation: await the first result, propagate rejection, and use the successful value to construct the next future.

```rust
// Example 22: Dependent asynchronous composition with immediately-ready fixtures.
use std::future::{Future, ready};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

async fn and_then_async<A, B, E, First, Next, Second>(first: First, next: Next) -> Result<B, E>
where
    First: Future<Output = Result<A, E>>,
    Next: FnOnce(A) -> Second,
    Second: Future<Output = Result<B, E>>,
{
    let value = first.await?;
    next(value).await
}

struct ReadyFixtureWake;

impl Wake for ReadyFixtureWake {
    fn wake(self: Arc<Self>) {}
}

fn main() {
    let waker = Waker::from(Arc::new(ReadyFixtureWake));
    let mut context = Context::from_waker(&waker);

    let successful = and_then_async(ready(Ok::<u32, &str>(7)), |value| {
        ready(Ok(value.to_string()))
    });
    let mut successful = Box::pin(successful);
    assert_eq!(
        successful.as_mut().poll(&mut context),
        Poll::Ready(Ok(String::from("7"))),
    );

    let mut calls = 0;
    let rejected = and_then_async(ready(Err::<u32, &str>("rejected")), |value| {
        calls += 1;
        ready(Ok(value))
    });
    let mut rejected = Box::pin(rejected);
    assert_eq!(
        rejected.as_mut().poll(&mut context),
        Poll::Ready(Err("rejected")),
    );
    drop(rejected);
    assert_eq!(calls, 0);
}
```

The no-op waker is appropriate only for this single-poll fixture, whose component futures are immediately ready. It is not an executor for pending work. `Wake` provides a safe interface for constructing a `Waker`; a real executor must arrange progress after notifications.[^wake]

The composition imposes no `Send` or `'static` bound because neither is needed merely to await these inputs in place. An enclosing execution API MUST add the capabilities its actual scheduling and lifetime model requires. The continuation is called at most once and only after success. Awaiting establishes dependency; it does not request an additional thread or independently spawn a task.[^async-functions][^future]

#### 13.6 Cancellation and lifetime contracts

An asynchronous component MUST document the state and externally visible progress that may exist when its future is dropped before completion. A future can own buffers, borrow mutable state, hold resource guards, or refer to independently executing work. Its cancellation behavior follows those concrete ownership and execution relationships.

Dropping a pending future does not reverse interactions already performed. Dropping a handle to independently executing work requires the semantics of that handle to be considered separately. A completion-dependent operation, such as publishing a completed record or acknowledging a request, SHOULD occur only after its prerequisite success has been established.

Cleanup that must report failure SHOULD be an explicit operation in the normal completion path. Destruction handles the fallback lifecycle specified by each owned type; it is not a replacement for observing a fallible completion result. Rust's destruction rules describe when destructors run, and operations such as forgetting a value demonstrate why resource protocols must state their execution assumptions rather than assert unconditional finalization.[^destructors][^forget]

#### 13.7 Laws across the boundary

A law about a pure plan compares plans or their specified interpretations. A law about an executing computation must additionally account for external observations, order, and partial progress. These are different equivalence relations and MUST NOT be substituted for each other.

For example, concatenating two report plans and rendering them can agree with concatenating their rendered bytes. Executing two plans separately can add an intermediate flush, while executing the concatenated plan may not. Those executions may be byte-equivalent on success but have different interaction traces. A valid optimization must preserve the observations required by its actual contract.

<a id="chapter-14"></a>
### 14. Local Effects and Mutable State

#### 14.1 The boundary of observation

A function can implement a pure value transformation using local mutation when that mutation cannot change observations outside the transformation. The book's treatment of local effects makes this a question of scope and observation, not a prohibition on assignment syntax.[^b14]

The relevant proof obligation is ownership of the mutable state and stability of the public result. Locally created storage may be updated while constructing an output. Exclusively owned input storage may be reused after ownership has been transferred into the function. A borrowed input must remain unchanged when the interface promises a value-preserving read.

A claim of local purity MUST include the behavior of callbacks, comparison functions, hashing, cloning, and destructors invoked during the computation. Local storage does not make an effectful callback pure. Likewise, a shallow copy of a handle to shared mutable state does not make the referenced state local.

#### 14.2 Consuming and borrowed transformations

A consuming interface can return the transformed owner without retaining an old snapshot. A borrowed interface that promises a separate result may copy the necessary data. Both can expose a pure value-level contract; the ownership and cost contracts differ.

```rust
// Example 23: Local mutation behind two explicit ownership interfaces.
fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.sort_unstable();
    values.dedup();
    values
}

fn sorted_unique_copy(values: &[String]) -> Vec<String> {
    sorted_unique(values.to_vec())
}

fn main() {
    let original = vec![
        String::from("beta"),
        String::from("alpha"),
        String::from("beta"),
    ];
    let snapshot = original.clone();
    let result = sorted_unique_copy(&original);
    assert_eq!(original, snapshot);
    assert_eq!(result, vec!["alpha", "beta"]);
    assert_eq!(sorted_unique(result.clone()), result);
    assert!(sorted_unique(Vec::new()).is_empty());
}
```

The output is sorted by the ordinary ordering of `String`, contains one representative of each distinct string, and has the same membership as the input. The consuming form reuses owned vector storage; the borrowed form explicitly clones the strings before that transformation. `dedup` removes consecutive duplicates, which is why sorting precedes it.[^slice][^vec]

A stable sort is necessary when the order of equal-key records carries meaning. In this example equal complete strings are interchangeable under the chosen value equality. The choice of unstable sorting does not justify discarding record identity or equal-key order in a different domain.

#### 14.3 A local-mutation argument

A review of a pure wrapper around mutation SHOULD establish four facts. First, every mutated object is created inside the computation or transferred into it with exclusive authority over the relevant state. Second, mutation cannot be observed through another owner, an escaping callback, a shared interior-mutable reference, or an external effect. Third, the resulting public value satisfies the promised invariant. Fourth, rejected inputs and algorithmic failures have the stated behavior.

A mutable helper may accept `&mut Buffer` because its caller owns a private construction buffer. That helper's interface is explicitly stateful. The enclosing function can nevertheless present a pure input-to-output contract when it keeps the construction state local. The proof applies to that enclosing use, not to every call of the helper.

Ownership of an outer container is not sufficient when its elements refer to shared state. A `Vec` of shared counters is not the same value abstraction as a `Vec` of integers. The semantic contents, not just the top-level binding, determine what can change.

#### 14.4 Builders and invariant restoration

A builder MAY use mutable storage to construct a validated result. Its public completion operation MUST either produce a value satisfying the invariant or return a defined rejection. Intermediate construction states SHOULD remain private when other code could mistake them for completed values.

The public representation must preserve the promised invariant after completion. Private fields and narrow accessors can make this straightforward. Calling a method `freeze` does not by itself establish immutability; the returned type's operations and any reachable aliases determine whether the value remains stable.

When a function accepts a caller-owned `&mut` value, it MUST state whether rejection preserves that value. Validation before mutation, construction of a replacement followed by assignment, and a documented partial-update protocol are different valid contracts. A failure-preservation guarantee MUST be tested at every rejection point.

#### 14.5 Interior mutability and memoization

Interior-mutability types permit mutation through shared references according to their own access rules.[^cell] Their use in a pure abstraction requires a semantic argument. A cache may preserve value-level behavior when it stores only a deterministic function of immutable inputs, does not expose cache state as part of the result, and has no externally visible initialization behavior.

A cache key MUST include every input that affects the cached result. Cached entries MUST remain valid for the lifetime during which they may be returned. A cache over changing input needs an explicit invalidation or versioning contract; it cannot be justified by the reasoning for an immutable-input cache.

Memoization SHOULD be introduced for a measured or structurally evident reuse opportunity. The memory retained by cached inputs, outputs, and shared nodes is part of the cost. A correct cache can still retain substantially more data than recomputation would require.

#### 14.6 Destruction and owned capabilities

Destruction of ordinary local value storage is normally outside the chosen domain observations. Destruction of a capability may unlock a mutex, release a file, signal another component, or otherwise participate in an execution protocol. A generic consuming combinator MUST NOT be described as observationally inert without accounting for such behavior in its input and captured types.

A persistent collection's destructor also has an operational cost. The destruction strategy for a long chain should be reviewed alongside its traversal and update strategies. Example 03 uses iterative release of its uniquely owned suffix to make that cost explicit. Resource-bearing payloads retain their own destruction contracts.

The functional design objective is not to hide these facts. It is to place them at interfaces where ownership, lifetime, and the relevant observations can be understood together.[^destructors]

<a id="chapter-15"></a>
### 15. Stream Processing and Incremental I/O

#### 15.1 Sources, transformations, and sinks

A **source** supplies input elements. A **stream transformation**, also called a **transducer**, incrementally turns input into output, possibly carrying state between elements. A **sink** consumes output. A complete streaming process connects these roles while specifying demand, failure, termination, and resource ownership. The book's final chapter develops this separation and treats early termination and resource release as composition requirements.[^b15]

A source may be a pure sequence generator or an effectful reader. A transformation may be a pure mapping, a stateful value transformation, or an effectful stage. These roles MUST be identified independently. An iterator over a file performs interactions when advanced; a transformation over an immutable slice may only compute values.

A streaming interface MUST distinguish an ordinary item, normal exhaustion, and failure. For example, `Iterator<Item = Result<T, E>>` uses `None` for exhaustion and `Some(Err(error))` for an error item. Whether the consumer stops at that error or continues is an additional policy. The type alone does not select it.

#### 15.2 A stateful transducer

A pure transition function can specify the state and output of each step. An iterator adapter can then keep that state locally and expose the resulting sequence. Here each Boolean input toggles or preserves a parity state, and each input produces exactly one output.

```rust
// Example 24: A pure step rule interpreted as an incremental transformation.
fn parity_step(state: bool, input: bool) -> (bool, bool) {
    let next = state ^ input;
    (next, next)
}

fn running_parity<I>(input: I) -> impl Iterator<Item = bool>
where
    I: IntoIterator<Item = bool>,
{
    input.into_iter().scan(false, |state, item| {
        let (output, next) = parity_step(*state, item);
        *state = next;
        Some(output)
    })
}

fn main() {
    assert_eq!(parity_step(false, true), (true, true));
    assert_eq!(parity_step(true, false), (true, true));
    let input = [true, false, true, true];
    assert_eq!(
        running_parity(input).collect::<Vec<_>>(),
        vec![true, true, false, true]
    );
    assert_eq!(
        running_parity(input).take(2).collect::<Vec<_>>(),
        vec![true, true]
    );
}
```

`scan` provides mutable access to its local state for each step, and its closure returns an optional output.[^iterator] This particular closure always returns `Some`, so the transformation emits exactly once per input and does not introduce an early-exhaustion signal. The underlying input's own termination and effect contract still applies.

The sequence specification can be stated independently of the adapter: start with `false`; for each input, apply `parity_step`; retain its first component as output and carry its second component to the following step. A collection-based reference implementation and the incremental implementation SHOULD agree on every finite input.

#### 15.3 Demand, buffering, and termination

A stage MUST state how much input it may inspect to produce output. A one-to-one map consumes one element per output. A filter may inspect several. A chunking stage may buffer multiple inputs. A grouping or sorting stage may need an entire group or collection. Calling all of these stages lazy does not give them the same memory or progress behavior.

A bounded-memory claim MUST include buffered elements, captured values, retained shared nodes, queues, and pending outputs. Limiting the number of elements is insufficient when an element can contain an unbounded byte string. Streaming protocols SHOULD bound record size, nesting, expansion, and accumulated diagnostics where the input can control those dimensions.

Backpressure means that downstream demand or capacity constrains upstream production. A pull-based iterator exposes a natural demand point at advancement, but an underlying source may prefetch or retain data. A concurrent stage needs explicit queue and in-flight-work limits. A buffering policy SHOULD state whether it preserves input order and how much completed work can wait behind an earlier unfinished item.

A terminal consumer MUST define whether it reads the whole input, stops after a bounded prefix, or stops at a predicate or failure. A short-circuiting consumer must leave every owned or borrowed resource in the state promised by its lifecycle contract. No later stage should rely on work that the consumer deliberately did not request.

#### 15.4 Text and chunk boundaries

Input chunks are transport units, not necessarily semantic records. A delimiter or multibyte text character may cross a chunk boundary. A record decoder MUST retain the partial state needed to reconstruct the same record regardless of how its bytes are partitioned, within its specified limits.

For a decoder that accepts UTF-8 records, validating each arbitrary transport chunk as an independent string is the wrong contract. The decoder SHOULD validate complete framed text or carry explicit decoding state between chunks. ASCII byte protocols can instead define their grammar directly over bytes. These are different formats and should be named accordingly.[^str]

The **chunk-partition law** requires equivalent results when the same byte sequence is split into different transport chunks, provided the sequence of bytes, end-of-input position, and any stated timing or resource assumptions remain the same. The law compares outputs, errors, and final decoder state. It does not identify a truncated input with a complete one.

#### 15.5 An end-to-end bounded-record pipeline

The following pipeline demonstrates the whole boundary. Each input record has the grammar `label,count`: the label contains 1–32 ASCII bytes, begins with a lowercase letter, and continues with lowercase letters, digits, underscores, or hyphens. The count contains one or more ASCII decimal digits and represents a nonzero `u32`. Leading zeroes are accepted. Spaces, signs, empty records, and extra fields are rejected.

Records end with LF or CRLF. A final record without a terminator is accepted; a lone trailing CR is not a terminator. The maximum record size is 4,096 bytes including any terminator. Output is `label=count` followed by LF, with the numeric value rendered without redundant leading zeroes. Processing stops at the first read, framing, UTF-8, parsing, arithmetic, or write error.

```rust
// Example 25: Bounded framing, pure parsing, checked state, and explicit I/O.
use std::io::{self, BufRead, Cursor, Read, Write};
use std::num::NonZeroU32;

const MAX_RECORD_BYTES: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecordError {
    Fields,
    Label,
    Count,
}

impl std::fmt::Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Fields => "record must contain label,count",
            Self::Label => "label does not match the ASCII grammar",
            Self::Count => "count must be a nonzero u32 in decimal digits",
        })
    }
}

impl std::error::Error for RecordError {}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    label: String,
    count: NonZeroU32,
}

fn parse_record(text: &str) -> Result<Record, RecordError> {
    let (label, count) = text.split_once(',').ok_or(RecordError::Fields)?;
    let bytes = label.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 32
        || !bytes[0].is_ascii_lowercase()
        || !bytes.iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(*byte, b'_' | b'-')
        })
    {
        return Err(RecordError::Label);
    }
    if count.is_empty() || !count.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(RecordError::Count);
    }
    let count = count
        .parse::<NonZeroU32>()
        .map_err(|_| RecordError::Count)?;
    Ok(Record {
        label: label.to_owned(),
        count,
    })
}

fn render_record(record: &Record) -> String {
    format!("{}={}\n", record.label, record.count.get())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Summary {
    records: u64,
    units: u64,
}

#[derive(Debug)]
enum PipelineError {
    Read(io::Error),
    RecordTooLong {
        line: u64,
    },
    Utf8 {
        line: u64,
        cause: std::str::Utf8Error,
    },
    Malformed {
        line: u64,
        cause: RecordError,
    },
    SummaryOverflow,
    Write(io::Error),
    Flush(io::Error),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(_) => f.write_str("failed to read input"),
            Self::RecordTooLong { line } => write!(f, "record {line} exceeds the size limit"),
            Self::Utf8 { line, .. } => write!(f, "record {line} is not valid UTF-8"),
            Self::Malformed { line, .. } => write!(f, "record {line} is malformed"),
            Self::SummaryOverflow => f.write_str("summary exceeds the u64 range"),
            Self::Write(_) => f.write_str("failed to write output"),
            Self::Flush(_) => f.write_str("failed to flush output"),
        }
    }
}

impl std::error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read(cause) | Self::Write(cause) | Self::Flush(cause) => Some(cause),
            Self::Utf8 { cause, .. } => Some(cause),
            Self::Malformed { cause, .. } => Some(cause),
            Self::RecordTooLong { .. } | Self::SummaryOverflow => None,
        }
    }
}

fn transform<R: BufRead, W: Write>(mut input: R, output: &mut W) -> Result<Summary, PipelineError> {
    let mut raw = Vec::new();
    let mut summary = Summary::default();

    loop {
        raw.clear();
        let count = input
            .by_ref()
            .take((MAX_RECORD_BYTES as u64) + 1)
            .read_until(b'\n', &mut raw)
            .map_err(PipelineError::Read)?;

        if count == 0 {
            output.flush().map_err(PipelineError::Flush)?;
            return Ok(summary);
        }

        let line = summary
            .records
            .checked_add(1)
            .ok_or(PipelineError::SummaryOverflow)?;
        if raw.len() > MAX_RECORD_BYTES {
            return Err(PipelineError::RecordTooLong { line });
        }
        if raw.last() == Some(&b'\n') {
            raw.pop();
            if raw.last() == Some(&b'\r') {
                raw.pop();
            }
        }

        let text =
            std::str::from_utf8(&raw).map_err(|cause| PipelineError::Utf8 { line, cause })?;
        let record =
            parse_record(text).map_err(|cause| PipelineError::Malformed { line, cause })?;
        let units = summary
            .units
            .checked_add(u64::from(record.count.get()))
            .ok_or(PipelineError::SummaryOverflow)?;
        let rendered = render_record(&record);
        output
            .write_all(rendered.as_bytes())
            .map_err(PipelineError::Write)?;
        summary = Summary {
            records: line,
            units,
        };
    }
}

struct FailAfter {
    limit: usize,
    bytes: Vec<u8>,
}

impl Write for FailAfter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.is_empty() {
            return Ok(0);
        }
        let remaining = self.limit.saturating_sub(self.bytes.len());
        if remaining == 0 {
            return Err(io::Error::new(io::ErrorKind::BrokenPipe, "fixture limit"));
        }
        let count = remaining.min(input.len());
        self.bytes.extend_from_slice(&input[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn main() {
    let input = Cursor::new(b"alpha,002\r\nbeta_1,3".as_slice());
    let mut output = Vec::new();
    assert_eq!(
        transform(input, &mut output).expect("valid fixture"),
        Summary {
            records: 2,
            units: 5
        },
    );
    assert_eq!(output, b"alpha=2\nbeta_1=3\n");

    let mut output = Vec::new();
    let error = transform(Cursor::new(b"alpha,2\nbeta,0\n".as_slice()), &mut output);
    assert!(matches!(
        error,
        Err(PipelineError::Malformed { line: 2, .. })
    ));
    assert_eq!(output, b"alpha=2\n");

    let mut output = Vec::new();
    let oversized = vec![b'x'; MAX_RECORD_BYTES + 1];
    let error = transform(Cursor::new(oversized), &mut output);
    assert!(matches!(
        error,
        Err(PipelineError::RecordTooLong { line: 1 })
    ));
    assert!(output.is_empty());

    let mut output = Vec::new();
    let error = transform(Cursor::new([0xff_u8, b'\n']), &mut output);
    assert!(matches!(error, Err(PipelineError::Utf8 { line: 1, .. })));

    let mut output = FailAfter {
        limit: 3,
        bytes: Vec::new(),
    };
    let error = transform(Cursor::new(b"alpha,2\n".as_slice()), &mut output);
    assert!(matches!(error, Err(PipelineError::Write(_))));
    assert_eq!(output.bytes, b"alp");
}
```

The pure functions are `parse_record` and `render_record`. Framing, reading, writing, and flushing belong to `transform`. The checked summary update is computed before the corresponding output write. It becomes the current summary only after `write_all` succeeds. A failed write can still leave a prefix of that record visible, as the final assertion specifies.

The per-record `take` adapter limits how many bytes `read_until` may append before the pipeline detects an oversized record. The logical length of `raw` is at most 4,097 bytes. Its allocation capacity may exceed its length; the transformation's working storage is proportional to the configured record bound, not to the total number of input records. The reader's buffering and the sink's retained output are separate costs. In particular, the `Vec<u8>` test sink intentionally retains all output.[^read][^take][^bufread]

This pipeline deliberately has no skip-invalid-record mode. A parser failure is not converted to exhaustion, a missing item, or a successful partial report. Empty input succeeds with an empty summary after a successful flush. Successful return means all records were accepted, all output writes completed according to the writer contract, and the final flush succeeded. It is not a statement about storage durability beyond that contract.[^write]

#### 15.6 Resource ownership and release

A streaming API MUST specify who owns the reader, writer, and any intermediate resource. Example 25 takes the reader by value and borrows the writer. When the function returns, its local ownership of the reader ends; the writer remains with the caller. Passing an owned reader and passing a borrowed reader therefore produce different release responsibilities even with the same generic signature.

A process SHOULD release resources as soon as it has finished using them. It MUST define normal exhaustion, source failure, transformation failure, sink failure, and early consumer termination. This is the practical consequence of the source and consumer finalization obligations emphasized in the book.[^b15]

Buffered writers require special attention because their destructor's behavior is not an error-reporting channel. An explicit flush or completion operation is required when successful delivery through that buffer is part of the success contract. A flush failure must not be converted into a successful result.[^bufwriter]

RAII supports release when the owning values are dropped along the relevant execution path. The lifecycle contract MUST state any assumptions about process termination, deliberate leaks, and cancellation. A resource's drop behavior and an externally required completion protocol are separate matters.[^destructors][^forget]

#### 15.7 Streaming laws and tests

A streaming transformation adopting a pure value contract MUST agree with its finite-sequence specification on finite inputs. Identity must preserve the sequence. Composition must preserve the stated order of transformations. A stateful transformation must carry the same transition state regardless of transport chunking. A bounded-prefix consumer must produce the specified prefix without demanding later outputs merely to construct that prefix.

Tests SHOULD cover empty input, a final unterminated record, each permitted terminator, a terminator split across reads, multibyte input split across reads, records at and beyond the configured bound, malformed data after a valid prefix, and numeric boundaries. Fault-injecting readers and writers SHOULD exercise read failures, short writes, write failures after partial progress, and final flush failure. Resource tests SHOULD observe release on normal completion, failure, and early stop.

A small-read wrapper is useful for testing chunk-partition invariance: give the same bytes to the pipeline through different positive maximum read sizes and compare the resulting output and outcome. Such a wrapper must preserve the reader contract. Deliberately returning zero from a non-exhausted fixture can instead simulate exhaustion and is not equivalent chunking.

#### 15.8 The complete design discipline

The completed design connects the same ideas introduced at the beginning: explicit inputs, validated values, transformations with defined failure, local state whose evolution can be inspected, and an interpreter that owns external progress. Combinators provide reuse where their contracts compose. Laws justify particular refactorings. Ownership determines which values are moved, borrowed, shared, or released. Resource bounds make the operational meaning explicit.

Functional programming in Rust is established by those properties together. A chain of combinator calls is not the criterion. A program is well factored when its decisions can be understood as value transformations, its dependencies are explicit, and every transition from describing work to performing work has a clear contract.

<a id="conformance"></a>
## Appendix A. Conformance and Review

### A.1 Conformance criteria

Conformance is assessed at a component's documented interface. An effectful adapter is not required to claim purity; it is required to identify and correctly implement its effects. A pure component must satisfy the stronger value-level obligations it adopts. An implementation MAY use a different representation from an example while preserving the applicable contracts.

| ID | Requirement | Required review evidence |
| --- | --- | --- |
| FP-01 | Pure interfaces expose all domain-relevant inputs and produce no external interaction. | Dependency review; tests using values rather than production capabilities. |
| FP-02 | Accepted domains, invariants, and unsuccessful outcomes are defined. | Constructor contracts; boundary cases; checked arithmetic and indexing arguments. |
| FP-03 | Ownership, borrowing, sharing, and cloning match the interface's semantics. | No unexplained snapshot assumption; explicit cost and aliasing analysis. |
| FP-04 | Callable bounds match actual invocation and access requirements. | Call count, capture lifetime, and callback-effect review. |
| FP-05 | Evaluation timing, consumption, short-circuiting, and retention are explicit. | Demand tests; first-error or all-error tests; buffer and traversal analysis. |
| FP-06 | State transitions expose final state and a failure-state policy. | Replay tests; accepted and rejected transition invariants. |
| FP-07 | Algebraic names are accompanied by their laws and equivalence relation. | Identity, composition, associativity, and other applicable law suites. |
| FP-08 | Numeric operations satisfy the laws claimed on the declared carrier. | Boundary tests; an arithmetic model; counterexample review for regrouping. |
| FP-09 | Parser composition defines cursor validity, commitment, diagnostics, and limits. | Full-consumption tests; choice tests; progress and input-budget checks. |
| FP-10 | Parallel or asynchronous boundaries specify ordering, failure, and progress. | Sequential-reference comparison; lifecycle tests; bounded-work policy. |
| FP-11 | I/O boundaries specify partial progress, completion, and release. | Fault injection; partial-write assertions; flush and early-stop tests. |

A component MUST satisfy each applicable requirement. “Not applicable” requires an explanation tied to the interface, not an absence of tests. A departure from a `SHOULD` recommendation should identify the concrete requirement that motivates it and the evidence that the alternative preserves the relevant contract.

### A.2 Documentation of a functional interface

The documentation for a reusable functional component SHOULD state its semantic role, inputs and outputs, invariant, ownership mode, failure behavior, evaluation policy, relevant laws, and significant resource costs. It SHOULD name any callback assumptions, especially purity, totality, and invocation order.

A concise contract can be organized as follows:

```text
Interface: <name and signature>
Role: <value transformation, state action, description, or interpreter>
Domain and invariant: <accepted inputs and guaranteed outputs>
Ownership: <moved, borrowed, shared, or copied information>
Outcome: <success, absence, rejection, and failure-state policy>
Evaluation: <eager or deferred; call counts; order; stopping behavior>
Observations and laws: <equivalence relation and applicable equations>
Resources: <allocation, retention, input limits, and release responsibility>
Evidence: <examples, generated tests, exhaustive cases, proofs, benchmarks>
```

Not every small private helper needs a separate contract template. The obligations still apply, and the enclosing interface may supply them. Repeating a law's name without its domain or operational assumptions is not adequate documentation.

### A.3 Maintenance

A change to error priority, traversal order, evaluation timing, snapshot behavior, or resource completion can be a semantic change even when the Rust signature remains identical. Such changes MUST be reviewed against the stated laws and execution contract.

A performance change SHOULD be measured against a realistic workload and checked against the same semantics as the original implementation. Reduced allocation is not an improvement when it changes ownership guarantees. Parallel speedup is not an improvement when it changes a required ordering. A smaller implementation is not an improvement when it drops a failure case.

Application code SHOULD reuse well-defined concrete interfaces and standard-library operations. A new abstraction should capture a recurring operation or an important domain boundary. Its benefit should be visible in simpler clients, stronger invariants, reusable laws, or a clearer execution model.

<a id="source-correspondence"></a>
## Appendix B. Correspondence with the Source Book

The following table records the conceptual basis of this document. Chapter titles and organization correspond to Manning's public book and liveBook pages. The table identifies subject correspondence; it does not attribute this document's Rust policies or examples to the authors.

| This document | Source chapter | Conceptual basis |
| --- | --- | --- |
| 1. Meaning and purity | Chapters 1 and 14 | Pure functions, substitution, separation of decisions and interactions, contextual observation. |
| 2. Functions and ownership | Chapter 2 | Higher-order functions, polymorphic functions, composition, partial application, type-guided design. |
| 3. Functional data structures | Chapter 3 | Algebraic data types, pattern matching, persistence, structural sharing, folds. |
| 4. Errors as values | Chapter 4 | Optional and fallible values, lifting, sequencing, traversal, centralized recovery policy. |
| 5. Strictness and laziness | Chapter 5 | Deferred evaluation, memoization, incremental transformations, unfolding, productivity. |
| 6. Explicit state | Chapter 6 | State transitions, returned next state, deterministic generation, compositional state actions. |
| 7. Parallel computation | Chapter 7 | Description versus execution, composable work, API laws, representation and scheduling consequences. |
| 8. Property-based testing | Chapter 8 | Properties, input generators, size control, counterexample reduction, reusable law suites. |
| 9. Parser combinators | Chapter 9 | Algebra-first design, sequencing, context dependence, structured diagnostics, commitment and backtracking. |
| 10. Monoids | Chapter 10 | Associativity, identity, folding, partitioning, homomorphisms, products of summaries. |
| 11. Functors and monadic composition | Chapter 11 | Structure-preserving mapping, dependent sequencing, identity and associativity laws. |
| 12. Applicative validation and traversal | Chapter 12 | Independent combination, accumulating errors, applicative laws, ordered traversal and explicit state. |
| 13. External effects | Chapter 13 | Factoring effects, descriptions, interpreters, pure model interpretations, asynchronous interaction. |
| 14. Local mutation | Chapter 14 | Scoped mutation, observable behavior, immutable results, contextual purity. |
| 15. Stream processing | Chapter 15 | Sources, transformations, sinks, incremental I/O, early termination, resource safety. |

The original examples, exercises, and narrative are not reproduced here. The Rust examples are independent constructions using the standard library and the ownership and execution contracts stated in this document.

<a id="terminology"></a>
## Appendix D. Terminology

| Term | Meaning in this document |
| --- | --- |
| Algebra | A collection of representations or types, operations, and laws governing those operations. |
| Algebraic data type | A type assembled from alternatives and products, represented here primarily by enums, structs, and tuples. |
| Applicative combination | Combining contextual inputs whose choice does not depend on another input's successful value. |
| Bind | Dependent sequencing; the first successful value determines the following contextual computation. |
| Combinator | An operation that constructs a value or computation from other values or computations according to a reusable rule. |
| Context | The surrounding interpretation of a value, such as absence, failure, ordered multiplicity, or explicit state. |
| Effect description | A value requesting an operation without itself performing that operation. |
| Fold | Combining a structure's elements into a result in a specified order and association. |
| Functorial mapping | Structure-preserving value transformation satisfying identity and composition. |
| Interpreter | A component that assigns an execution or model meaning to a description. |
| Monoid | A carrier, a closed associative binary operation, and a two-sided identity under a stated equivalence. |
| Persistence | Retention of earlier logical versions after an update produces a new version. |
| Productive generation | Generation in which each requested next output can be obtained in finite work under the stated assumptions. |
| Pure function | A value transformation determined by explicit input and fixed immutable captures, with no external observation beyond its result in the stated model. |
| Referential transparency | Substitutability of an expression by an equivalent computed value relative to an observation model. |
| Semigroup | A carrier with a closed associative binary operation under a stated equivalence. |
| Sequence | An operation that turns a structure of contextual values into a contextual structure. |
| State action | A computation whose explicit meaning includes an input state, a result, and a final state. |
| Structural sharing | Reusing storage for unchanged portions of logically distinct versions. |
| Totality | Producing a defined result for every input in the declared domain. |
| Transducer | An incremental input-to-output transformation, possibly with state carried between input elements. |
| Traversal | Applying a contextual operation across a structure while preserving the structure and using a specified combination policy. |

<a id="references"></a>
## References

Book references identify the conceptual source. Rust references identify language and standard-library contracts. The normative policies and examples in this document are independently authored. Online references were reviewed on September 14, 2026; API use in the examples targets the baseline stated in the overview rather than every feature shown in the current documentation.

[^book]: Paul Chiusano and Rúnar Bjarnason, [*Functional Programming in Scala*](https://www.manning.com/books/functional-programming-in-scala), first edition, Manning Publications, September 2014, ISBN 9781617290657. The [public liveBook introduction](https://livebook.manning.com/book/functional-programming-in-scala) describes the four-part organization. Chapter links below identify conceptual correspondence; this review did not have the complete book or verify printed page ranges.
[^b01]: Chiusano and Bjarnason, [Chapter 1, “What is functional programming?”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-1). Pure functions, referential transparency, substitution, and factoring external interaction from domain decisions.
[^b02]: Chiusano and Bjarnason, [Chapter 2, “Getting started with functional programming in Scala”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-2). Higher-order and polymorphic functions, partial application, composition, and following types to implementations.
[^b03]: Chiusano and Bjarnason, [Chapter 3, “Functional data structures”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-3). Algebraic data types, pattern matching, persistent structures, sharing, and folds.
[^b04]: Chiusano and Bjarnason, [Chapter 4, “Handling errors without exceptions”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-4). Representing unsuccessful outcomes as values and composing them with higher-order operations.
[^b05]: Chiusano and Bjarnason, [Chapter 5, “Strictness and laziness”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-5). Deferred evaluation, memoization, incremental transformation, unfolding, and productivity.
[^b06]: Chiusano and Bjarnason, [Chapter 6, “Purely functional state”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-6). State actions returning a value and a next state, deterministic generation, and state composition.
[^b07]: Chiusano and Bjarnason, [Chapter 7, “Purely functional parallelism”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-7). Computation descriptions, interpreters, API laws, and execution-dependent implementation choices.
[^b08]: Chiusano and Bjarnason, [Chapter 8, “Property-based testing”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-8). Properties, generators, size policies, counterexample minimization, and testing laws.
[^b09]: Chiusano and Bjarnason, [Chapter 9, “Parser combinators”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-9). Algebra-first design, sequencing, diagnostics, commitment, and backtracking.
[^b10]: Chiusano and Bjarnason, [Chapter 10, “Monoids”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-10). Associativity, identity, folds, parallel decomposition, and composed summaries.
[^b11]: Chiusano and Bjarnason, [Chapter 11, “Monads”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-11). Functor laws, dependent sequencing, monad laws, and their concrete interpretations.
[^b12]: Chiusano and Bjarnason, [Chapter 12, “Applicative and traversable functors”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-12). Independent combination, validation, applicative laws, and traversal.
[^b13]: Chiusano and Bjarnason, [Chapter 13, “External effects and I/O”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-13). Effect factoring, descriptions, and interpreters.
[^b14]: Chiusano and Bjarnason, [Chapter 14, “Local effects and mutable state”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-14). Local mutation and contextual purity.
[^b15]: Chiusano and Bjarnason, [Chapter 15, “Stream processing and incremental I/O”](https://livebook.manning.com/book/functional-programming-in-scala/chapter-15). Producer, consumer, and resource-finalization obligations.
[^edition]: Rust Blog, [Announcing Rust 1.85.0 and Rust 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/).
[^closures-book]: *The Rust Programming Language*, [Closures: Anonymous Functions that Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html).
[^closures-reference]: *The Rust Reference*, [Closure types](https://doc.rust-lang.org/reference/types/closure.html).
[^fn-once]: Rust standard library, [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html).
[^fn-mut]: Rust standard library, [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html).
[^fn]: Rust standard library, [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html).
[^expressions]: *The Rust Reference*, [Expressions](https://doc.rust-lang.org/reference/expressions.html), including operand evaluation order.
[^operators]: *The Rust Reference*, [Operator expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html), including integer overflow and error-propagation expressions.
[^enums]: *The Rust Reference*, [Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html).
[^higher-ranked]: *The Rust Reference*, [Higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds).
[^clone]: Rust standard library, [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).
[^cell]: Rust standard library, [`std::cell`](https://doc.rust-lang.org/std/cell/index.html).
[^once-cell]: Rust standard library, [`OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html).
[^rc]: Rust standard library, [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), including shared ownership and `try_unwrap`.
[^cow]: Rust standard library, [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html).
[^slice]: Rust standard library, [Slice methods](https://doc.rust-lang.org/std/primitive.slice.html), including sorting and borrowed access.
[^vec]: Rust standard library, [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), including deduplication, capacity, and ownership operations.
[^str]: Rust standard library, [`str`](https://doc.rust-lang.org/std/primitive.str.html), including UTF-8 text, byte lengths, character boundaries, and parsing.
[^option]: Rust standard library, [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html).
[^result]: Rust standard library, [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html).
[^option-collection]: Rust standard library, [`std::option`](https://doc.rust-lang.org/std/option/index.html), “Collecting into Option,” and `Option`'s `FromIterator` implementation.
[^result-collection]: Rust standard library, [`std::result`](https://doc.rust-lang.org/std/result/index.html), iteration and collection behavior, and `Result`'s `FromIterator` implementation.
[^nonzero]: Rust standard library, [`NonZero`](https://doc.rust-lang.org/std/num/struct.NonZero.html), including construction, value access, and concrete integer parsing implementations.
[^error]: Rust standard library, [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html).
[^iterators-book]: *The Rust Programming Language*, [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html).
[^iter-module]: Rust standard library, [`std::iter`](https://doc.rust-lang.org/std/iter/index.html).
[^iterator]: Rust standard library, [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
[^double-ended]: Rust standard library, [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html), including `rfold`.
[^from-fn]: Rust standard library, [`from_fn`](https://doc.rust-lang.org/std/iter/fn.from_fn.html).
[^successors]: Rust standard library, [`successors`](https://doc.rust-lang.org/std/iter/fn.successors.html).
[^fused]: Rust standard library, [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html).
[^wrapping]: Rust standard library, [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html).
[^float]: Rust standard library, [`f64`](https://doc.rust-lang.org/std/primitive.f64.html), including floating-point behavior and NaN.
[^send]: Rust standard library, [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html).
[^sync]: Rust standard library, [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html).
[^thread-scope]: Rust standard library, [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html).
[^thread-builder]: Rust standard library, [`thread::Builder`](https://doc.rust-lang.org/std/thread/struct.Builder.html), including fallible scoped spawning.
[^scoped-join]: Rust standard library, [`ScopedJoinHandle`](https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html).
[^join-handle]: Rust standard library, [`JoinHandle`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html), including joining and handle destruction.
[^async-functions]: *The Rust Reference*, [Functions](https://doc.rust-lang.org/reference/items/functions.html), “Async functions.”
[^future]: Rust standard library, [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html).
[^wake]: Rust standard library, [`Wake`](https://doc.rust-lang.org/std/task/trait.Wake.html).
[^destructors]: *The Rust Reference*, [Destructors](https://doc.rust-lang.org/reference/destructors.html).
[^forget]: Rust standard library, [`mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html), including resource-leak and destructor assumptions.
[^read]: Rust standard library, [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html).
[^take]: Rust standard library, [`io::Take`](https://doc.rust-lang.org/std/io/struct.Take.html).
[^bufread]: Rust standard library, [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html), including `read_until`.
[^write]: Rust standard library, [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html), including `write_all` and `flush`.
[^bufwriter]: Rust standard library, [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html), including flushing and errors during destruction.
[^rustc-cli]: *The rustc book*, [Command-line arguments](https://doc.rust-lang.org/rustc/command-line-arguments.html).
[^rustc-codegen]: *The rustc book*, [Codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html), including `opt-level` and `overflow-checks`.
