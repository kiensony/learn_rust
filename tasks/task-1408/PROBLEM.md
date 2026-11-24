# task-1408 — Millisecond remainder · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the milliseconds remaining after whole seconds.

Return the absolute differences between each adjacent pair, in order. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn millisecond_remainder_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `millisecond_remainder_steps(&[])` gives `vec![]`.
- `millisecond_remainder_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1408/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1408`.

[Module reading](https://doc.rust-lang.org/std/time/)
