# task-2605 — Embedded checksum · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Adapt a provided no_std device primitive to a host-side trace. For each byte rotate each byte one bit right by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_checksum_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `embedded_checksum_changes(&[])` gives `vec![]`.
- `embedded_checksum_changes(&[4])` gives `vec![2u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2605/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2605`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
