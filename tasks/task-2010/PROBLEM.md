# task-2010 — Polled lengths · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, the byte itself.

Return the observations in reverse order. Retain duplicate observations. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_lengths_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_lengths_reverse(&[])` gives `vec![]`.
- `polled_lengths_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2010/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2010`.

[Module reading](https://doc.rust-lang.org/std/future/)
