# task-2002 — Polled lengths · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, the byte itself.

Return the sum of all observations, widening each one before adding. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_lengths_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_lengths_total(&[])` gives `0`.
- `polled_lengths_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2002/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2002`.

[Module reading](https://doc.rust-lang.org/std/future/)
