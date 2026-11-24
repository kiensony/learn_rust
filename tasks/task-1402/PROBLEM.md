# task-1402 — Millisecond remainder · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the milliseconds remaining after whole seconds.

Return the sum of all observations, widening each one before adding. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn millisecond_remainder_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `millisecond_remainder_total(&[])` gives `0`.
- `millisecond_remainder_total(&[4])` gives `68`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1402/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1402`.

[Module reading](https://doc.rust-lang.org/std/time/)
