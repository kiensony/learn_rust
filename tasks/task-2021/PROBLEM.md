# task-2021 — Polled groups · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the byte divided by four, rounded down.

Return the observations in input order, including duplicates. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_groups_trace(input: &[u8]) -> Vec<u32>
```

## Public examples

- `polled_groups_trace(&[])` gives `vec![]`.
- `polled_groups_trace(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2021/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2021`.

[Module reading](https://doc.rust-lang.org/std/future/)
