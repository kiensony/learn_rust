# task-2621 — Embedded parity · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte compute bit parity by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return the observations in input order, including duplicates. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_parity_trace(input: &[u8]) -> Vec<u32>
```

## Public examples

- `embedded_parity_trace(&[])` gives `vec![]`.
- `embedded_parity_trace(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2621/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2621`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
