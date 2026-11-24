# task-2012 — Polled flags · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create one ready future per byte and poll each exactly once. Observe the ready payload, one for nonzero, zero otherwise.

Return the sum of all observations, widening each one before adding. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_flags_total(input: &[u8]) -> u64
```

## Public examples

- `polled_flags_total(&[])` gives `0`.
- `polled_flags_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2012/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2012`.

[Module reading](https://doc.rust-lang.org/std/future/)
