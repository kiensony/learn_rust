# task-2622 — Embedded parity · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Adapt a provided no_std device primitive to a host-side trace. For each byte compute bit parity by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return the sum of all observations, widening each one before adding. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_parity_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `embedded_parity_total(&[])` gives `0`.
- `embedded_parity_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2622/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2622`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
