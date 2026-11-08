# AGENTS.md — Cordion

Guidance for AI coding agents contributing to **Cordion**, a Rust library for building Discord bots.

**This document defines development philosophy and decision-making principles, not a concrete architecture or API specification.** Do not infer types, crate boundaries, module layouts, naming conventions, registration mechanisms, or implementation status from it. The current repository, tests, and explicit maintainer instructions are the source of truth.

## 1. Purpose

Cordion aims to make Discord bot development **ergonomic, strongly typed, fast, and resource-efficient**, especially in CPU and memory usage.

The library should absorb complexity that users would otherwise face when working with the Discord API. It must not replace that complexity with unnecessary Cordion-specific conventions.

**Guiding principle: Make the user's application simpler without hiding important behavior or sacrificing correctness.**

## 2. Design philosophy

### Optimize for the user of the library

- Judge an API by the experience of writing, reading, debugging, and maintaining an application that uses it—not just by how easy it is to implement internally.
- Prefer idiomatic Rust and predictable behavior over library-specific rituals, boilerplate, or surprising implicit actions.
- Avoid forcing users to understand incidental dependencies, implementation details, or conversions that add no meaningful value.
- Make common operations straightforward while keeping advanced needs possible without fighting the abstraction.

### Use types when they earn their complexity

- Use Rust's type system to clarify intent and prevent invalid operations when the benefit is substantial.
- Do not introduce a wrapper, trait, generic parameter, or state machine merely because it is possible.
- Weigh compile-time guarantees against API complexity, discoverability, diagnostics, and maintenance cost.
- When a property cannot be validated at compile time, make runtime behavior and errors clear.

### Prefer Rust's existing vocabulary

- Before adding a dependency or a Cordion-specific type, check whether `std` already expresses the requirement naturally.
- Reuse familiar `String`, `&str`, `Option`, `Result`, `Duration`, `Instant`, `SystemTime`, slices, and standard collections where their semantics fit.
- Keep wire representations distinct from convenient application representations: conversion at a protocol boundary is often better than exposing raw API formats everywhere.
- Third-party crates are welcome when they provide meaningful functionality missing from `std`. Do not make users install or understand them merely to express common concepts.

### Keep the public contract independent of internals

- Do not expose implementation-specific dependencies or backend details through the public API without a deliberate reason.
- Prefer stable, understandable boundaries that allow internal implementations to change without forcing consumers to rewrite their applications.
- Add abstraction where there is a meaningful reason for substitutability, not everywhere by default.
- Avoid making optional functionality, heavy infrastructure, or unrelated subsystems prerequisites for ordinary use.

### Respect performance without guessing

- Treat latency, CPU time, memory usage, allocations, and unnecessary work as important design costs.
- Evaluate costs on real execution paths, including ordinary and high-load usage; do not optimize solely for theoretical elegance.
- Prefer simple, efficient approaches first. Benchmark or profile before claiming that a more complex approach is faster.
- Do not exchange correctness or developer experience for an insignificant, unmeasured gain.

### Preserve correctness and room for change

- The Discord API is an external, evolving protocol. Model its documented behavior accurately rather than designing around assumptions.
- Consider how an implementation behaves when fields are absent, values are null, unfamiliar values appear, or a response differs by context.
- Avoid silently discarding information or rejecting reasonable future protocol extensions without a clear need.
- Distinguish protocol restrictions from restrictions introduced only by Cordion. Do not unnecessarily narrow what consumers can do.
- Favor incremental changes that remain understandable and testable over speculative frameworks built for hypothetical future features.

## 3. Questions to ask before implementing

Consider these questions for each meaningful design decision:

1. **User experience:** What concrete inconvenience does this solve? Is there a simpler API with fewer steps or concepts?
2. **Correctness:** Is the behavior supported by the current Discord specification? What are the relevant edge cases and failure modes?
3. **Typing:** Does an additional type prevent a realistic mistake, or just create conversions and friction?
4. **Boundaries:** Is an internal implementation choice leaking into a public commitment?
5. **Compatibility:** How will this behave if Discord introduces a new field, value, or capability?
6. **Performance:** What does this cost in CPU, memory, allocations, contention, and complexity? Do measurements support the trade-off?
7. **Maintainability:** Is the design easy to reason about, test, extend, and replace? Is any special machinery truly necessary?
8. **Surprise:** Are there hidden side effects, implicit assumptions, unexpected resource use, or misleading error messages?

These are review questions, **not a requirement to maximize every property at once**. Identify trade-offs explicitly instead of silently prioritizing one objective.

## 4. Code-level examples and review heuristics

The following pairs illustrate the **reasoning expected of agents**, not required Cordion APIs. Names such as `State`, `User`, and `Id` are illustrative; some blocks are API-signature sketches rather than complete compilable programs. The repository decides actual types, dependencies, and public signatures. Prefer the right-hand approach **when its stated assumptions hold**; do not mechanically replace valid code.

### A. Use `std::time` before reaching for `chrono` or `time`

**Questionable when all you need is a timestamp or an interval:**

```rust
pub fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
pub fn retry_after(&self) -> time::Duration;
```

**Prefer standard, interoperable types for standard concepts:**

```rust
pub fn created_at(&self) -> std::time::SystemTime;
pub fn retry_after(&self) -> std::time::Duration;
```

For elapsed time or timeouts, prefer `Instant` and `Duration`:

```rust
use std::time::{Duration, Instant};
let started = Instant::now();
let timeout = Duration::from_secs(5);
let elapsed = started.elapsed();
```

`Instant` measures elapsed time and must not be treated as a persistable wall-clock timestamp. `SystemTime` represents wall-clock time but is not monotonic; handle errors from `duration_since` rather than assuming time can never go backward. Use `chrono`, `time`, or another dependency when calendar arithmetic, time zones, RFC 3339 parsing/formatting, or another genuinely needed capability justifies it. **The rule is “do not burden users unnecessarily,” not “never use dependencies.”**

### B. Do not leak incidental backend error types

**Avoid if Redis is only an internal storage choice:**

```rust
pub fn load_state(&self) -> redis::RedisResult<State>;
```

`redis::RedisResult<T>` aliases `Result<T, redis::RedisError>`. A higher-level library that exposes it forces callers to know Redis, convert Redis errors, and possibly change code if the backend changes. Redis itself is justified in exposing its own domain errors; **the issue is a different library exporting its incidental dependency**.

**Illustrative alternative when a general-purpose error is sufficient:**

```rust
pub fn load_state(&self) -> anyhow::Result<State>;
```

Retain source errors and attach diagnostic context at meaningful boundaries; do not swallow the cause just to erase its type. A library-owned typed error is justified when callers must reliably distinguish actionable failure categories. Do not create an extra alias such as `CordionResult<T>` solely to rename an existing `Result`.

### C. Distinguish absence from failure

**Avoid silently turning an operational error into “not found”:**

```rust
fn find_key(&self) -> Option<String> {
    self.try_find_key().ok().flatten()
}
```

**If lookup can fail independently of whether an item exists:**

```rust
fn find_key(&self) -> anyhow::Result<Option<String>>;
```

`Ok(Some(value))` means found; `Ok(None)` means successfully checked but absent; `Err(error)` means the lookup failed. This is preferable to treating an outage as a legitimate missing value. Conversely, an already-loaded optional field may need only `Option<T>`—do not insert `Result` into stored data without a reason.

### D. Prefer useful domain types, not wrappers for their own sake

**Questionable when no invariant or semantics are added:**

```rust
pub struct Username(String);
impl Username {
    pub fn as_str(&self) -> &str { &self.0 }
}
```

A plain `String` or `&str` is often simpler. But a type **can** earn its cost by preventing a realistic error. For example, two bare integers can be accidentally swapped:

```rust
fn ban_user(guild_id: u64, user_id: u64);
ban_user(user_id, guild_id); // Type-checks, but the arguments are reversed.
```

**A typed-ID approach would catch that class of mistake:**

```rust
fn ban_user(guild_id: Id<Guild>, user_id: Id<User>);
// Passing Id<User> as the first argument would be a type error.
```

These are examples of a criterion, **not** instructions to implement those specific names or signatures. Introduce types for safety, semantics, validation, and discoverability; reject types that merely add `.into()` and `.get()` calls everywhere.

### E. Do not use magic sentinel values when absence is a state

**Avoid:**

```rust
let last_seen: i64 = -1; // -1 means there was no previous event.
if last_seen != -1 { /* ... */ }
```

**Prefer expressing the state directly:**

```rust
let last_seen: Option<usize> = None;
if let Some(index) = last_seen { /* ... */ }
```

This avoids hidden conventions, casts, and accidental comparisons with sentinels. A protocol-defined sentinel can still be appropriate **at the wire boundary**; convert it into the clearest application representation where useful.

### F. Match Serde defaults to the protocol's semantics

**Avoid when “missing” does not mean “false”:**

```rust
#[derive(serde::Deserialize)]
struct Account {
    #[serde(default)]
    verified: bool,
}
```

**Preserve the distinction if the field is genuinely optional:**

```rust
#[derive(serde::Deserialize)]
struct Account {
    verified: Option<bool>,
}
```

For a field where Discord explicitly defines absence as equivalent to `false`, `#[serde(default)]` may be a deliberate ergonomic choice. For others, replacing missing, null, unknown, or inaccessible data with a default changes meaning. Check the **specific field's** documentation; do not apply one rule to every optional field. Similarly, avoid `#[serde(deny_unknown_fields)]` on evolving external objects unless strict rejection is required.

### G. Avoid assuming the set of Discord values is closed

**Avoid rejecting values merely because they are not yet known:**

```rust
fn decode_tier(raw: u8) -> Result<Tier, Error> {
    match raw {
        0 => Ok(Tier::None),
        1 => Ok(Tier::Basic),
        _ => Err(Error::UnknownTier(raw)),
    }
}
```

**For extensible protocol fields, consider retaining the raw value:**

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
struct Tier(u8);
impl Tier {
    const NONE: Self = Self(0);
    const BASIC: Self = Self(1);
    fn raw(self) -> u8 { self.0 }
}
```

This can preserve an unfamiliar value without discarding it. A closed enum is still appropriate when the protocol **really guarantees** a closed set or an unknown value is genuinely invalid. Make that distinction explicit.

### H. Do not assume in-memory representation equals JSON representation

A strongly typed Snowflake ID may be stored as an integer, while the Discord API normally encodes Snowflakes as decimal **strings** in JSON (with documented edge cases). Do not assume a derived `Serialize` implementation gets the wire format right.

**Test the actual public serialization contract:**

```rust
// Example assertion; `id` is any correctly implemented Snowflake-ID value.
assert_eq!(
    serde_json::to_value(&id).unwrap(),
    serde_json::json!("1556204224512000000"),
);
```

Test both serialize and deserialize, boundary values, and the documented behavior of missing or null fields. This is **not** a requirement to represent every ID internally as a `String`.

### I. Avoid panics on ordinary external input

**Avoid:**

```rust
let flags = payload["flags"].as_u64().unwrap();
let kind = match raw_kind {
    1 => "first",
    2 => "second",
    _ => unreachable!("Discord will never add another kind"),
};
```

**Prefer checked decoding and explicit outcomes:**

```rust
let flags = payload.get("flags").and_then(|v| v.as_u64());
// Interpret absence separately from malformed input if the contract requires it.
```

`unwrap` and `expect` are appropriate for proven internal invariants or tests, not routine network failures or untrusted payloads. Do not quietly convert malformed data into a normal default just to avoid returning an error.

### J. Borrow data instead of allocating by default

**Avoid a forced allocation when a borrow is sufficient:**

```rust
pub fn username(&self) -> String {
    self.username.clone()
}
```

**Prefer:**

```rust
pub fn username(&self) -> &str {
    &self.username
}
```

Likewise, prefer `&[T]` to `&Vec<T>` in an API that only needs to read a sequence:

```rust
fn inspect(ids: &[u64]); // Accepts arrays, Vecs, and slices by borrowing.
```

Use owned results, `Clone`, and allocations when required by ownership, lifetimes, threading, or intended API semantics; do not turn performance advice into awkward lifetime-heavy APIs without benefit.

### K. Do not add synchronization or indirection without a requirement

**Questionable for local, single-owner data:**

```rust
let buffer = std::sync::Arc::new(
    std::sync::Mutex::new(Vec::<u8>::new())
);
```

**Prefer ordinary ownership when sufficient:**

```rust
let mut buffer = Vec::<u8>::new();
```

Similarly, do not use `Box<dyn Trait>`, an async trait, or several layers of factories for a single implementation without a useful substitution point. Traits are worthwhile for **actual interchangeable boundaries** (for example I/O or storage), testing needs, or substantial API clarity. Avoid globally shared mutable state when explicit ownership works.

### L. Avoid abstraction and registration machinery that burden unrelated users

Do not force downstream users to understand a custom framework or register features just to use ordinary model types. An implementation convenience is not automatically a public-API benefit.

**Questionable when the trait has one trivial implementation and no consumer benefit:**

```rust
trait NameProvider {
    fn name(&self) -> &str;
}
// Only ever implemented for one concrete model, solely to call `.name()`.
```

**Often sufficient:**

```rust
impl User {
    fn name(&self) -> &str { &self.username }
}
```

Likewise, judge derive macros, registries, `Deref` impls, and implicit behavior by what they save **users**, not only by how clever their implementation is. If an abstraction exists solely to compensate for a past architectural decision, reconsider that decision first. Do not lock in any particular registration or module strategy based on this document.

### M. Use advanced Rust features for a demonstrated benefit

**Avoid `unsafe` only to bypass a check without proving the invariant:**

```rust
let item = unsafe { items.get_unchecked(index) };
```

**Prefer a safe operation unless profiling and proof justify otherwise:**

```rust
let item = items.get(index); // Handle `None` as appropriate.
```

When `unsafe` is genuinely needed, document its safety invariants and validate them. Likewise, do not add clever `Deref`, pervasive `Copy`/`Clone`, type-level states, macros, or lifetime gymnastics without weighing correctness, diagnostics, readability, and real performance benefits. Type-level guarantees are valuable when they prevent meaningful misuse; “more types” is not automatically “better API.”

### N. Keep responsibilities cohesive, not monolithic or atomized

A single enormous model file that mixes a type, every companion data type, flags, validation, and protocol glue is hard to navigate. Splitting each trivial field or getter into its own module is equally unhelpful. Group code by a **cohesive concept with its own logic**, and split when the additional structure improves navigation or testing. This is a criterion for reviewing existing code, **not** a prescribed directory tree.

**Review shortcut:** Ask what application code using this API looks like. If the “better” implementation saves internal effort but makes applications more awkward, that is a reason to reconsider it.

## 5. Consult the current Discord documentation

**Verify relevant Discord API details against official sources before implementing or changing protocol-related behavior.** Do not rely on memory, third-party libraries, old code, or search snippets as the sole authority.

Preferred lookup process:

1. Consult the official documentation index: <https://docs.discord.com/llms.txt>.
2. Follow the relevant Markdown (`.md`) page linked from the index. Discover the current URL rather than guessing paths.
3. If the Markdown view is incomplete, ambiguous, or inaccessible, consult the official documentation source: <https://github.com/discord/discord-api-docs>. Source documents may use **MDX**; imports and referenced components may contain important context.
4. Check the official change log for recent changes and deprecations: <https://docs.discord.com/developers/change-log>.
5. Follow references to related official pages. Use repository history or discussions as supporting context when necessary, while distinguishing discussion from documented behavior.

During implementation, verify applicable details such as field presence and nullability, data representations, response variants, validation rules, permissions and intents, protocol state transitions, rate limits, and error behavior.

If sources disagree or cannot be accessed, **state the uncertainty**. Do not invent protocol guarantees to complete an implementation.

## 6. Engineering practices

- Read the relevant source, manifests, tests, and existing conventions before editing. Do not assume a planned feature or design already exists.
- Preserve established project decisions unless there is a concrete reason to revisit them. Distinguish existing behavior, proposals, and unresolved questions.
- Make focused, reviewable changes. Avoid unrelated refactors and speculative generalization.
- Prefer straightforward, idiomatic Rust. Justify advanced mechanisms and `unsafe` with clear benefits, invariants, and safety reasoning.
- Avoid unnecessary cloning, allocations, synchronization, and retention of data; also avoid contorting simple code without evidence of a real performance problem.
- Do not panic on ordinary external input, network failures, or other expected runtime conditions. Preserve useful error context.
- Consider network reliability, cancellation, reconnection, rate limiting, and unexpected payloads in the code responsible for them.
- Keep credentials and sensitive data out of logs, fixtures, examples, and commits.
- Keep documentation consistent with actual behavior. Do not present intended features as implemented.

## 7. Tooling, validation, and communication

**Use these Rust tools by default.** Do not substitute other formatters, linters, or test runners for routine project checks without a specific reason or maintainer instruction:

- **Formatter:** `rustfmt`, invoked through `cargo +nightly-2026-10-08 fmt --all` (check with `cargo +nightly-2026-10-08 fmt --all -- --check`).
- **Linter:** **Clippy**, invoked through `cargo clippy --workspace --all-targets`. Treat warnings seriously; for a strict check, use `cargo clippy --workspace --all-targets -- -D warnings`. Only add `--all-features` when the feature configuration supports building all features together.
- **Test runner:** **cargo-nextest**, invoked through `cargo nextest run --workspace`. Use this instead of `cargo test` for normal unit and integration test execution.
- **Documentation tests:** nextest does not execute Rust doctests; run `cargo test --workspace --doc` separately when doctests are relevant.

If a required tool is not installed or cannot run in the environment, report that limitation instead of silently substituting a different tool or claiming validation succeeded.

- Test behavior at the appropriate level: normal paths, boundary conditions, failures, and compatibility cases. For protocol-facing changes, use documented examples and representative fixtures where feasible.
- Prefer deterministic tests. Never require real Discord credentials or live requests for ordinary unit tests.
- Respect the repository's toolchain and configuration. Run the relevant formatting, build, Clippy, and nextest checks when available; run broader checks for changes with broader impact.
- Report what was checked and what could not be checked. Never imply that tests or documentation were verified when they were not.
- Explain user-visible behavior changes, compatibility implications, and unresolved trade-offs.
- Communicate with the maintainer in **Japanese**, unless instructed otherwise. Keep Rust identifiers and upstream terminology unchanged.

**Default bias:** Minimize the burden on Cordion's users. Introduce complexity only when it meaningfully improves correctness, ergonomics, performance, or maintainability.
