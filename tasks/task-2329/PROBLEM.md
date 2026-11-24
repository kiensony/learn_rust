# task-2329 — Batch medians · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The input is a fixed sequence of benchmark durations in microseconds. Observe the lower median in each consecutive group of up to three measurements. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn batch_medians_longest_run(input: &[u8]) -> u64
```

## Public examples

- `batch_medians_longest_run(&[])` gives `0`.
- `batch_medians_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2329/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2329`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
