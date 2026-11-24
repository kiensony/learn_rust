# task-1416 — Elapsed seconds · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Count adjacent pairs for which the second observation is strictly smaller. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `elapsed_seconds_descending_steps(&[])` gives `0`.
- `elapsed_seconds_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1416/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1416`.

[Module reading](https://doc.rust-lang.org/std/time/)
