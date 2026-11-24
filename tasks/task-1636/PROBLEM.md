# task-1636 — Clamp property · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: clamping to [4, 16] changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Count adjacent pairs for which the second observation is strictly smaller. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn clamp_property_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `clamp_property_descending_steps(&[])` gives `0`.
- `clamp_property_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1636/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1636`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
