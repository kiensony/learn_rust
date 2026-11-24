# task-1638 — Clamp property · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: clamping to [4, 16] changes the value. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return the absolute differences between each adjacent pair, in order. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn clamp_property_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `clamp_property_steps(&[])` gives `vec![]`.
- `clamp_property_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1638/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1638`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
