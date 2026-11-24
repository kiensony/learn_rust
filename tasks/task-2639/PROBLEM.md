# task-2639 — Embedded saturation · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte double each u8 using saturation by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_saturation_longest_run(input: &[u8]) -> u64
```

## Public examples

- `embedded_saturation_longest_run(&[])` gives `0`.
- `embedded_saturation_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2639/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2639`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
