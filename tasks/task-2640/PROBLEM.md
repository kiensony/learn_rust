# task-2640 — Embedded saturation · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte double each u8 using saturation by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return the observations in reverse order. Retain duplicate observations. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_saturation_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `embedded_saturation_reverse(&[])` gives `vec![]`.
- `embedded_saturation_reverse(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2640/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2640`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
