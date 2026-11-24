# task-1797 — Generated offset · run ledger

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte increased by the generated value (currently 7). Do not hard-code the generated value.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use a Cargo build script and env!. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn generated_offset_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `generated_offset_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(10u32, 1usize), (13u32, 1usize), (16u32, 1usize), (19u32, 1usize), (28u32, 1usize), (22u32, 1usize), (7u32, 1usize), (33u32, 1usize), (15u32, 1usize), (38u32, 1usize)]`.
- `generated_offset_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1797/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1797`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
