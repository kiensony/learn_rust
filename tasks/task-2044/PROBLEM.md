# task-2044 — Polled costs · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the number of set bits.

Keep the first observation and every later one strictly above all earlier observations. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_costs_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `polled_costs_peaks(&[])` gives `vec![]`.
- `polled_costs_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2044/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2044`.

[Module reading](https://doc.rust-lang.org/std/future/)
