# task-2038 — Polled masks · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the low three bits.

Return the absolute differences between each adjacent pair, in order. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_masks_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `polled_masks_steps(&[])` gives `vec![]`.
- `polled_masks_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2038/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2038`.

[Module reading](https://doc.rust-lang.org/std/future/)
