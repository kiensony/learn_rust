# task-1549 — Packet Gray code · frequent readings

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reflected binary Gray code x XOR (x shifted right by one) from each byte. During review locate the package, import and filesystem names in the manifest.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use the bundled path dependency without duplicating its decoder. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn packet_gray_code_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `packet_gray_code_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![2u32, 5u32, 13u32]`.
- `packet_gray_code_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1549/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1549`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
