# task-2013 — Polled flags · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, one for nonzero, zero otherwise.

Return Some((minimum, maximum)) or None for an empty trace. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_flags_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_flags_bounds(&[])` gives `None`.
- `polled_flags_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2013/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2013`.

[Module reading](https://doc.rust-lang.org/std/future/)
