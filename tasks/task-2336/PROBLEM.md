# task-2336 — Batch variation · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Count adjacent pairs for which the second observation is strictly smaller. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `batch_variation_descending_steps(&[])` gives `0`.
- `batch_variation_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2336/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2336`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
