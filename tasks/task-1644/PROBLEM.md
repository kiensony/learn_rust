# task-1644 — Shift property · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: shifting right then left by one changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Keep the first observation and every later one strictly above all earlier observations. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn shift_property_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shift_property_peaks(&[])` gives `vec![]`.
- `shift_property_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1644/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1644`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
