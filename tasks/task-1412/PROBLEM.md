# task-1412 — Elapsed seconds · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the whole elapsed seconds.

Return the sum of all observations, widening each one before adding. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn elapsed_seconds_total(input: &[u8]) -> u64
```

## Public examples

- `elapsed_seconds_total(&[])` gives `0`.
- `elapsed_seconds_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1412/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1412`.

[Module reading](https://doc.rust-lang.org/std/time/)
