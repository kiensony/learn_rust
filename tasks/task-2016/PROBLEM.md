# task-2016 — Polled flags · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, one for nonzero, zero otherwise.

Count adjacent pairs for which the second observation is strictly smaller. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_flags_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_flags_descending_steps(&[])` gives `0`.
- `polled_flags_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2016/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2016`.

[Module reading](https://doc.rust-lang.org/std/future/)
