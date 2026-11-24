# task-2022 — Polled groups · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the byte divided by four, rounded down.

Return the sum of all observations, widening each one before adding. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_groups_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_groups_total(&[])` gives `0`.
- `polled_groups_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2022/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2022`.

[Module reading](https://doc.rust-lang.org/std/future/)
