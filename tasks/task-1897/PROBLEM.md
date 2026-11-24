# task-1897 — Weak read handles · run ledger

Start a fresh Arc<u8> per input byte. Observe the number of explicit weak observers after making x modulo 4 observers. Strong and weak handles have separate ownership semantics.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use Arc and explicit drop scopes. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn weak_read_handles_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `weak_read_handles_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(3u32, 1usize), (2u32, 1usize), (1u32, 1usize), (0u32, 1usize), (1u32, 1usize), (3u32, 1usize), (0u32, 1usize), (2u32, 1usize), (0u32, 1usize), (3u32, 1usize)]`.
- `weak_read_handles_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1897/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1897`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
