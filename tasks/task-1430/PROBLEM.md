# task-1430 — Remaining budget · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Return the observations in reverse order. Retain duplicate observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `remaining_budget_reverse(&[])` gives `vec![]`.
- `remaining_budget_reverse(&[4])` gives `vec![18u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1430/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1430`.

[Module reading](https://doc.rust-lang.org/std/time/)
