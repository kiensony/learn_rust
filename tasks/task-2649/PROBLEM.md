# task-2649 — Embedded saturation · frequent readings

Adapt a provided no_std device primitive to a host-side trace. For each byte double each u8 using saturation by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use the local no_std device library and a separate host test harness. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn embedded_saturation_frequent_readings(input: &[u8], keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `embedded_saturation_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3)` gives `vec![6u32, 12u32, 18u32]`.
- `embedded_saturation_frequent_readings(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2649/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2649`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
