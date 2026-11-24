# task-1547 — Packet low nibble · run ledger

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the lower four bits from each byte. During review locate the package, import and filesystem names in the manifest.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use the bundled path dependency without duplicating its decoder. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn packet_low_nibble_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `packet_low_nibble_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(3u32, 1usize), (6u32, 1usize), (9u32, 1usize), (12u32, 1usize), (5u32, 1usize), (15u32, 1usize), (0u32, 1usize), (10u32, 1usize), (8u32, 1usize), (15u32, 1usize)]`.
- `packet_low_nibble_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1547/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1547`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
