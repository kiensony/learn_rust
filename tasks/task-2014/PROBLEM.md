# task-2014 — Polled flags · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, one for nonzero, zero otherwise.

Keep the first observation and every later one strictly above all earlier observations. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_flags_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_flags_peaks(&[])` gives `vec![]`.
- `polled_flags_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2014/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2014`.

[Module reading](https://doc.rust-lang.org/std/future/)
