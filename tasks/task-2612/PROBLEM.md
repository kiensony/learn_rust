# task-2612 — Embedded flags · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte extract the lower four bits by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return the sum of all observations, widening each one before adding. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_flags_total(input: &[u8]) -> u64
```

## Public examples

- `embedded_flags_total(&[])` gives `0`.
- `embedded_flags_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2612/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2612`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
