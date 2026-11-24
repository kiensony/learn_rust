# task-2049 — Polled masks · frequent readings

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the low three bits.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use a pinned future and an explicit polling context. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn polled_masks_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `polled_masks_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![7u32, 0u32, 3u32]`.
- `polled_masks_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2049/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2049`.

[Module reading](https://doc.rust-lang.org/std/future/)
