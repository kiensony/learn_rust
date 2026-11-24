# task-2339 — Batch variation · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_longest_run(input: &[u8]) -> u64
```

## Public examples

- `batch_variation_longest_run(&[])` gives `0`.
- `batch_variation_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2339/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2339`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
