# task-1997 — Atomic bit union · run ledger

Start an AtomicU32 at zero. For each byte perform fetch_or with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use AtomicU32 read-modify-write operations and Ordering::Relaxed. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn atomic_bit_union_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `atomic_bit_union_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(0u32, 1usize), (3u32, 1usize), (7u32, 1usize), (15u32, 2usize), (31u32, 5usize)]`.
- `atomic_bit_union_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1997/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1997`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
