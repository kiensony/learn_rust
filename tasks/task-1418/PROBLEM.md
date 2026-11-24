# task-1418 — Elapsed seconds · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Return the absolute differences between each adjacent pair, in order. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `elapsed_seconds_steps(&[])` gives `vec![]`.
- `elapsed_seconds_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1418/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1418`.

[Module reading](https://doc.rust-lang.org/std/time/)
