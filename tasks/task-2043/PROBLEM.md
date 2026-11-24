# task-2043 — Polled costs · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the number of set bits.

Return Some((minimum, maximum)) or None for an empty trace. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_costs_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `polled_costs_bounds(&[])` gives `None`.
- `polled_costs_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2043/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2043`.

[Module reading](https://doc.rust-lang.org/std/future/)
