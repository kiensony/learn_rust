# task-2247 — Borrowed offsets · run ledger

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe zero-based byte offsets of LF terminators. Allocate only storage needed for the observations or result.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use borrowed slices or indices into the original input. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn borrowed_offsets_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `borrowed_offsets_run_ledger("a\nblue\nblue\r\nz\n")` gives `vec![(1u32, 1usize), (6u32, 1usize), (12u32, 1usize), (14u32, 1usize)]`.
- `borrowed_offsets_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2247/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2247`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
