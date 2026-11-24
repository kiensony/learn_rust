# task-2647 — Embedded flags · run ledger

Adapt a provided no_std device primitive to a host-side trace. For each byte extract the lower four bits by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use the local no_std device library and a separate host test harness. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn embedded_flags_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `embedded_flags_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(3u32, 1usize), (6u32, 1usize), (9u32, 1usize), (12u32, 1usize), (5u32, 1usize), (15u32, 1usize), (0u32, 1usize), (10u32, 1usize), (8u32, 1usize), (15u32, 1usize)]`.
- `embedded_flags_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2647/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2647`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
