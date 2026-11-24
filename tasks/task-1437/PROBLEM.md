# task-1437 — Timer windows · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the number of started 100-millisecond windows, with zero windows for zero duration.

Return distinct observations in ascending order. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn timer_windows_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `timer_windows_distinct(&[])` gives `vec![]`.
- `timer_windows_distinct(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1437/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1437`.

[Module reading](https://doc.rust-lang.org/std/time/)
