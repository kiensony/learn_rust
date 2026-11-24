# task-2335 — Batch variation · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `batch_variation_changes(&[])` gives `vec![]`.
- `batch_variation_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2335/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2335`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
