# task-1415 — Elapsed seconds · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `elapsed_seconds_changes(&[])` gives `vec![]`.
- `elapsed_seconds_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1415/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1415`.

[Module reading](https://doc.rust-lang.org/std/time/)
