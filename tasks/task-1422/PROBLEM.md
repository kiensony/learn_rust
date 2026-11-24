# task-1422 — Remaining budget · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Return the sum of all observations, widening each one before adding. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remaining_budget_total(&[])` gives `0`.
- `remaining_budget_total(&[4])` gives `18`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1422/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1422`.

[Module reading](https://doc.rust-lang.org/std/time/)
