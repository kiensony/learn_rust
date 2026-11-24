# task-2005 — Polled lengths · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create one ready future per byte and poll each exactly once. Observe the ready payload, the byte itself.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_lengths_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_lengths_changes(&[])` gives `vec![]`.
- `polled_lengths_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2005/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2005`.

[Module reading](https://doc.rust-lang.org/std/future/)
