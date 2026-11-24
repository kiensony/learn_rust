# task-1998 — Atomic toggle · rolling load

Start an AtomicU32 at zero. For each byte perform fetch_xor with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use AtomicU32 read-modify-write operations and Ordering::Relaxed. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn atomic_toggle_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `atomic_toggle_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![0u64, 3u64, 5u64, 12u64, 0u64, 21u64, 26u64, 26u64, 0u64, 8u64]`.
- `atomic_toggle_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1998/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1998`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
