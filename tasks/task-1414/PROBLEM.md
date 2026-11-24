# task-1414 — Elapsed seconds · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Keep the first observation and every later one strictly above all earlier observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `elapsed_seconds_peaks(&[])` gives `vec![]`.
- `elapsed_seconds_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1414/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1414`.

[Module reading](https://doc.rust-lang.org/std/time/)
