# task-1996 — Atomic running total · threshold crossing

Start an AtomicU32 at zero. For each byte perform fetch_add with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use AtomicU32 read-modify-write operations and Ordering::Relaxed. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn atomic_running_total_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `atomic_running_total_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 0))`.
- `atomic_running_total_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1996/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1996`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
