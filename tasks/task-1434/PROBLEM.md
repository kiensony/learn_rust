# task-1434 — Timer windows · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the number of started 100-millisecond windows, with zero windows for zero duration.

Keep the first observation and every later one strictly above all earlier observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn timer_windows_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `timer_windows_peaks(&[])` gives `vec![]`.
- `timer_windows_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1434/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1434`.

[Module reading](https://doc.rust-lang.org/std/time/)
