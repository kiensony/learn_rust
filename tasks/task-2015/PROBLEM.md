# task-2015 — Polled flags · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create one ready future per byte and poll each exactly once. Observe the ready payload, one for nonzero, zero otherwise.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_flags_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `polled_flags_changes(&[])` gives `vec![]`.
- `polled_flags_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2015/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2015`.

[Module reading](https://doc.rust-lang.org/std/future/)
