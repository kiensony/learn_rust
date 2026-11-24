# task-1397 — Frame offsets · run ledger

Decode length-prefixed frames. One unsigned byte gives each payload length. A zero-length frame is valid. Stop at the first incomplete frame. For each complete frame observe its zero-based offset of the length byte.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use slice range checks before consuming a frame. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn frame_offsets_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `frame_offsets_run_ledger(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2])` gives `vec![(0u32, 1usize), (3u32, 1usize), (5u32, 1usize), (6u32, 1usize)]`.
- `frame_offsets_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1397/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1397`.

[Module reading](https://doc.rust-lang.org/std/net/)
