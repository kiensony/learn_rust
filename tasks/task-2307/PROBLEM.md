# task-2307 — Warmup discard · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The input is a fixed sequence of benchmark durations in microseconds. Observe the measured durations after discarding the first two warmup iterations. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return distinct observations in ascending order. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn warmup_discard_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `warmup_discard_distinct(&[])` gives `vec![]`.
- `warmup_discard_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2307/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2307`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
