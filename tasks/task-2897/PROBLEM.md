# task-2897 — Big endian registers · run ledger

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the big-endian unsigned 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use fixed-width integer operations independent of host byte order. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn big_endian_registers_run_ledger(input: &[u8]) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `big_endian_registers_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31])` gives `vec![(774u32, 1usize), (2316u32, 1usize), (5391u32, 1usize), (26u32, 1usize), (2079u32, 1usize)]`.
- `big_endian_registers_run_ledger(&[])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2897/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2897`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
