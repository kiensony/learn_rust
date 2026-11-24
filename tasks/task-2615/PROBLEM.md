# task-2615 — Embedded flags · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Adapt a provided no_std device primitive to a host-side trace. For each byte extract the lower four bits by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_flags_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `embedded_flags_changes(&[])` gives `vec![]`.
- `embedded_flags_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2615/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2615`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
