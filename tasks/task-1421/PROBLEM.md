# task-1421 — Remaining budget · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Return the observations in input order, including duplicates. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_trace(input: &[u8]) -> Vec<u32>
```

## Public examples

- `remaining_budget_trace(&[])` gives `vec![]`.
- `remaining_budget_trace(&[4])` gives `vec![18u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1421/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1421`.

[Module reading](https://doc.rust-lang.org/std/time/)
