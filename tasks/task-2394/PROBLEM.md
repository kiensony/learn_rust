# task-2394 — Last change age · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Collect a cumulative trace over a byte stream. At each position observe the length of the current run of consecutive equal bytes. Use a single pass with constant extra bookkeeping beyond the result. Add a slow prefix-by-prefix oracle to a test and compare on a stream of at least 100 entries. Explain the number of element visits in each implementation.

Keep the first observation and every later one strictly above all earlier observations. Use incremental state and an independent performance model. During review, point to the operation that enforces its contract.

```rust
pub fn last_change_age_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `last_change_age_peaks(&[])` gives `vec![]`.
- `last_change_age_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2394/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2394`.

[Module reading](https://doc.rust-lang.org/std/time/struct.Instant.html)
