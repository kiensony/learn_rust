# task-1633 — Clamp property · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: clamping to [4, 16] changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return Some((minimum, maximum)) or None for an empty trace. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn clamp_property_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `clamp_property_bounds(&[])` gives `None`.
- `clamp_property_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1633/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1633`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
