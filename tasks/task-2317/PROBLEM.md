# task-2317 — Paired speedup · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The input is a fixed sequence of benchmark durations in microseconds. Observe nonnegative improvements in complete before/after pairs, old minus new with zero as the floor. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return distinct observations in ascending order. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn paired_speedup_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `paired_speedup_distinct(&[])` gives `vec![]`.
- `paired_speedup_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2317/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2317`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
