# task-2603 — Embedded checksum · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte rotate each byte one bit right by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Return Some((minimum, maximum)) or None for an empty trace. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_checksum_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `embedded_checksum_bounds(&[])` gives `None`.
- `embedded_checksum_bounds(&[4])` gives `Some((2u32, 2u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2603/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2603`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
