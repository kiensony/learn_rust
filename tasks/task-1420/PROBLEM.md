# task-1420 — Elapsed seconds · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Return the observations in reverse order. Retain duplicate observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `elapsed_seconds_reverse(&[])` gives `vec![]`.
- `elapsed_seconds_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1420/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1420`.

[Module reading](https://doc.rust-lang.org/std/time/)
