# task-2147 — Priority removal · run ledger

A scheduler receives one u8 key per entry. Observe the priority queue's removal order, largest key first, including duplicates. Explain the cost of inserting and extracting the next item.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use the standard library's BinaryHeap. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn priority_removal_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `priority_removal_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(31u32, 1usize), (26u32, 1usize), (21u32, 1usize), (15u32, 1usize), (12u32, 1usize), (9u32, 1usize), (8u32, 1usize), (6u32, 1usize), (3u32, 1usize), (0u32, 1usize)]`.
- `priority_removal_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2147/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2147`.

[Module reading](https://doc.rust-lang.org/std/collections/)
