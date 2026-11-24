# task-1444 — Deadline slack · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the absolute difference in milliseconds from a 250-millisecond deadline.

Keep the first observation and every later one strictly above all earlier observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn deadline_slack_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `deadline_slack_peaks(&[])` gives `vec![]`.
- `deadline_slack_peaks(&[4])` gives `vec![182u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1444/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1444`.

[Module reading](https://doc.rust-lang.org/std/time/)
