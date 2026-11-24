# task-2334 — Batch variation · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The input is a fixed sequence of benchmark durations in microseconds. Observe the range (largest minus smallest duration) in consecutive nonempty batches of four. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Keep the first observation and every later one strictly above all earlier observations. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_variation_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `batch_variation_peaks(&[])` gives `vec![]`.
- `batch_variation_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2334/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2334`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
