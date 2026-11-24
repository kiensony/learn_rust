# task-2497 — Reverse raw reads · run ledger

Advanced pointer-contract lab. Observe all bytes in reverse order. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use slice::as_ptr, pointer equality, and safe indexed access. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn reverse_raw_reads_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `reverse_raw_reads_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(31u32, 1usize), (8u32, 1usize), (26u32, 1usize), (0u32, 1usize), (15u32, 1usize), (21u32, 1usize), (12u32, 1usize), (9u32, 1usize), (6u32, 1usize), (3u32, 1usize)]`.
- `reverse_raw_reads_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2497/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2497`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
